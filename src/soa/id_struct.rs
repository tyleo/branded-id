use crate::{
    Id, IdVec, Scalar, UsizeId,
    soa::{IdRemap, IdStructIter, IdStructRawParts},
};
use std::{
    fmt::{self, Debug},
    hash::{Hash, Hasher},
};

/// An id pool that hands out and recycles typed integer handles.
///
/// Ids are allocated with [`retain`](Self::retain) and recycled with
/// [`release`](Self::release). Every id ever handed out lives in a single
/// `dense` list partitioned by [`len`](Self::len): `dense[..len]` are
/// the retained ids, packed, and `dense[len..]` are released ids waiting to
/// be recycled. Releasing swap-removes from the retained region so iteration
/// only ever visits retained ids.
///
/// The pool is keyed by the brand `TBrand` alone; the integer width it stores
/// indices in is the separate `TNum` parameter, which defaults to `u32`. So
/// `IdStruct<BFoo>` hands out [`U32Id<BFoo>`](crate::U32Id) while
/// `IdStruct<BFoo, usize>` hands out [`UsizeId<BFoo>`](crate::UsizeId); see
/// [`Scalar`] for the width-to-id mapping.
pub struct IdStruct<TBrand: ?Sized, TNum: Scalar = u32> {
    /// Every id ever handed out. `dense[..live_count]` are retained and
    /// packed; `dense[live_count..]` are released, with the next id to be
    /// recycled at `dense[live_count]`.
    dense: Vec<TNum::Id<TBrand>>,

    /// Per-id: its index in `dense`, stored in the pool's integer width `TNum`
    /// so the reverse index is no wider than it needs to be (e.g. `u32` for a
    /// `u32`-keyed pool). Valid for every id handed out; a freed id keeps
    /// pointing at its slot in the released region.
    sparse: IdVec<TBrand, TNum>,

    /// The number of retained ids, i.e. the boundary between the retained and
    /// released regions of `dense`.
    live_count: usize,
}

impl<TBrand: ?Sized, TNum: Scalar> IdStruct<TBrand, TNum> {
    /// Creates an empty id pool.
    pub const fn new() -> Self {
        Self {
            dense: Vec::new(),
            sparse: IdVec::new(),
            live_count: 0,
        }
    }

    /// Exposes the internal lists for advanced usage.
    pub fn as_raw_parts(&self) -> IdStructRawParts<'_, TBrand, TNum> {
        let (live, free) = self.dense.split_at(self.live_count);
        IdStructRawParts {
            live,
            sparse: &self.sparse,
            free,
        }
    }

    /// Releases every retained id and resets the pool to empty.
    pub fn clear(&mut self) {
        self.dense.clear();
        self.sparse.as_mut_vec().clear();
        self.live_count = 0;
    }

    /// Compacts the pool so its retained ids become the contiguous range
    /// `0..len`, and returns an [`IdRemap`] recording where each old id went.
    ///
    /// The id currently iterated `i`-th is relabeled to id `i`, so iteration
    /// order is preserved, just renumbered. The recycled ids waiting in the
    /// free region are discarded, so afterward the pool holds no released ids
    /// and the next [`retain`](Self::retain) hands out [`len`](Self::len).
    ///
    /// Because the live ids are renumbered, any id stored outside the pool is
    /// now stale. Translate each one through [`IdRemap::new_id`], and pass the
    /// returned remap to [`IdField::gc`](super::IdField::gc) on every paired
    /// field to move its values to the relabeled ids. Do this before retaining
    /// or releasing any further ids, while the fields are still in sync with
    /// the pre-gc layout the remap describes.
    pub fn gc(&mut self) -> IdRemap<TBrand, TNum> {
        let new_len = self.live_count;
        let old_len = self.sparse.len();

        // Record, per old id, the id it is relabeled to: the id at live index
        // `i` becomes id `i`. Ids not in the live region stay `None`.
        let mut new_ids: Vec<Option<TNum::Id<TBrand>>> = vec![None; old_len];
        for i in 0..new_len {
            let old = self.dense[i].to_usize_id().to_usize();
            let new = <TNum::Id<TBrand> as Id>::from_usize_id(UsizeId::from_usize(i));
            new_ids[old] = Some(new);
        }

        // Rebuild `dense` and `sparse` as the identity over `0..new_len`,
        // dropping the recycled free region: every live id now sits at its own
        // index, so both lists read `0, 1, .., new_len - 1`.
        self.dense.clear();
        let sparse = self.sparse.as_mut_vec();
        sparse.clear();
        for i in 0..new_len {
            self.dense.push(<TNum::Id<TBrand> as Id>::from_usize_id(
                UsizeId::from_usize(i),
            ));
            sparse.push(TNum::from_usize(i));
        }

        IdRemap::from_parts(IdVec::from_vec(new_ids), new_len)
    }

    /// Whether the pool currently has no retained ids.
    pub fn is_empty(&self) -> bool {
        self.live_count == 0
    }

    /// Whether `id` is currently retained. Safe and `false` for ids that were
    /// never handed out or have already been released.
    pub fn is_retained(&self, id: TNum::Id<TBrand>) -> bool {
        let id = id.to_usize_id();
        id.to_usize() < self.sparse.len() && self.sparse[id].to_usize() < self.live_count
    }

    /// Iterates the retained ids in their packed `live` order, the same as
    /// `(&self).into_iter()`.
    pub fn iter(&self) -> IdStructIter<'_, TNum::Id<TBrand>> {
        self.into_iter()
    }

    /// The number of ids currently retained from this pool.
    pub fn len(&self) -> usize {
        self.live_count
    }

    /// Peeks at the next id [`retain`](Self::retain) would return, without
    /// actually retaining it.
    pub fn peek_next(&self) -> TNum::Id<TBrand> {
        if self.live_count < self.dense.len() {
            self.dense[self.live_count]
        } else {
            self.peek_next_fresh()
        }
    }

    /// Peeks at the next id that would be freshly allocated, ignoring the
    /// released ids available for recycling.
    pub fn peek_next_fresh(&self) -> TNum::Id<TBrand> {
        <TNum::Id<TBrand> as Id>::from_usize_id(self.sparse.end())
    }

    /// Releases `id`, recycling it for a future [`retain`](Self::retain) and
    /// swap-removing it from the packed retained region of `dense`.
    ///
    /// # Panics
    /// Panics if `id` is not currently retained, including when the pool is
    /// empty.
    pub fn release(&mut self, id: TNum::Id<TBrand>) {
        let usize_id = id.to_usize_id();
        let index_backing = self.sparse[usize_id];
        let index = index_backing.to_usize();

        let last_live = self
            .live_count
            .checked_sub(1)
            .expect("released an id from an empty pool");

        // Move the last retained id into the released id's slot, keeping
        // `dense[..live_count]` packed, then drop the released id into the
        // vacated boundary slot so it sits at the front of the released
        // region. The two `sparse` writes swap the stored positions as-is, so
        // no usize round-trip is needed; and when `id` is already the last
        // retained id every write is a self-assignment, so no special case is
        // needed either.
        let last_id = self.dense[last_live];
        let last_id_usize = last_id.to_usize_id();
        let last_live_backing = self.sparse[last_id_usize];

        self.dense[index] = last_id;
        self.sparse[last_id_usize] = index_backing;

        self.dense[last_live] = id;
        self.sparse[usize_id] = last_live_backing;

        self.live_count = last_live;
    }

    /// Retains and returns an id, reusing a previously released id when one
    /// is available and otherwise allocating a fresh one.
    pub fn retain(&mut self) -> TNum::Id<TBrand> {
        let id = if self.live_count < self.dense.len() {
            // Recycle the id at the front of the released region. Its `sparse`
            // entry already points at this slot, so growing the retained
            // region by one is all that is needed to mark it retained.
            self.dense[self.live_count]
        } else {
            // Allocate a brand-new id, growing both lists in lock-step. The
            // new id lands at index `live_count`, and its own value is that
            // index, so `sparse` records that index as the id's position.
            let index = self.sparse.end();
            let id = <TNum::Id<TBrand> as Id>::from_usize_id(index);
            self.sparse.push(TNum::from_usize(index.to_usize()));
            self.dense.push(id);
            id
        };

        self.live_count += 1;

        id
    }
}

impl<TBrand: ?Sized, TNum: Scalar> Clone for IdStruct<TBrand, TNum> {
    fn clone(&self) -> Self {
        Self {
            dense: self.dense.clone(),
            sparse: self.sparse.clone(),
            live_count: self.live_count,
        }
    }
}

impl<TBrand: ?Sized, TNum: Scalar> Debug for IdStruct<TBrand, TNum>
where
    TNum::Id<TBrand>: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `sparse` is the inverse permutation of `dense`, so it adds nothing
        // beyond the retained (`live`) and recycled-next (`free`) partitions of
        // `dense` and is left out; only the ids need to be `Debug`, not `TNum`.
        let (live, free) = self.dense.split_at(self.live_count);
        f.debug_struct("IdStruct")
            .field("live", &live)
            .field("free", &free)
            .finish()
    }
}

impl<TBrand: ?Sized, TNum: Scalar> Default for IdStruct<TBrand, TNum> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TBrand: ?Sized, TNum: Scalar> Eq for IdStruct<TBrand, TNum> {}

impl<TBrand: ?Sized, TNum: Scalar> Hash for IdStruct<TBrand, TNum>
where
    TNum::Id<TBrand>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the same data `PartialEq` compares; `sparse` is derived from
        // `dense` so omitting it keeps Hash and Eq in agreement.
        self.dense.hash(state);
        self.live_count.hash(state);
    }
}

impl<'a, TBrand: ?Sized, TNum: Scalar> IntoIterator for &'a IdStruct<TBrand, TNum> {
    type Item = TNum::Id<TBrand>;
    type IntoIter = IdStructIter<'a, TNum::Id<TBrand>>;

    fn into_iter(self) -> Self::IntoIter {
        IdStructIter::from_live(&self.dense[..self.live_count])
    }
}

impl<TBrand: ?Sized, TNum: Scalar> PartialEq for IdStruct<TBrand, TNum> {
    fn eq(&self, other: &Self) -> bool {
        // Structural equality over the full internal layout: `dense` (whose
        // inverse `sparse` is redundant) plus the live/free boundary. Two pools
        // that retain the same ids but reached that state through a different
        // release history compare unequal.
        self.live_count == other.live_count && self.dense == other.dense
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        self.live_count != other.live_count || self.dense != other.dense
    }
}
