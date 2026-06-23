use crate::{
    Id, IdVec,
    soa::{IdStructIter, IdStructRawParts},
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
/// The pool is generic over the id type `TId`: see
/// [`U32IdStruct`](super::U32IdStruct) for the common `u32`-keyed
/// specialization.
pub struct IdStruct<TId: Id> {
    /// Every id ever handed out. `dense[..live_count]` are retained and
    /// packed; `dense[live_count..]` are released, with the next id to be
    /// recycled at `dense[live_count]`.
    dense: Vec<TId>,

    /// Per-id: its index in `dense`, stored in the id's backing integer
    /// ([`Id::Backing`]) so the reverse index is no wider than it needs to be
    /// (e.g. `u32` for a `u32`-keyed pool). Valid for every id handed out; a
    /// freed id keeps pointing at its slot in the released region.
    sparse: IdVec<TId::Brand, TId::Backing>,

    /// The number of retained ids, i.e. the boundary between the retained and
    /// released regions of `dense`.
    live_count: usize,
}

impl<TId: Id> IdStruct<TId> {
    /// Creates an empty id pool.
    pub const fn new() -> Self {
        Self {
            dense: Vec::new(),
            sparse: IdVec::new(),
            live_count: 0,
        }
    }

    /// Exposes the internal lists for advanced usage.
    pub fn as_raw_parts(&self) -> IdStructRawParts<'_, TId> {
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

    /// Whether the pool currently has no retained ids.
    pub fn is_empty(&self) -> bool {
        self.live_count == 0
    }

    /// Whether `id` is currently retained. Safe and `false` for ids that were
    /// never handed out or have already been released.
    pub fn is_retained(&self, id: TId) -> bool {
        let id = id.to_usize_id();
        id.to_usize() < self.sparse.len() && Self::index_of(self.sparse[id]) < self.live_count
    }

    /// Iterates the retained ids in their packed `live` order, the same as
    /// `(&self).into_iter()`.
    pub fn iter(&self) -> IdStructIter<'_, TId> {
        self.into_iter()
    }

    /// The number of ids currently retained from this pool.
    pub fn len(&self) -> usize {
        self.live_count
    }

    /// Peeks at the next id [`retain`](Self::retain) would return, without
    /// actually retaining it.
    pub fn peek_next(&self) -> TId {
        if self.live_count < self.dense.len() {
            self.dense[self.live_count]
        } else {
            self.peek_next_fresh()
        }
    }

    /// Peeks at the next id that would be freshly allocated, ignoring the
    /// released ids available for recycling.
    pub fn peek_next_fresh(&self) -> TId {
        TId::from_usize_id(self.sparse.end())
    }

    /// Releases `id`, recycling it for a future [`retain`](Self::retain) and
    /// swap-removing it from the packed retained region of `dense`.
    pub fn release(&mut self, id: TId) {
        let usize_id = id.to_usize_id();
        let index_backing = self.sparse[usize_id];
        let index = Self::index_of(index_backing);

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
    pub fn retain(&mut self) -> TId {
        let id = if self.live_count < self.dense.len() {
            // Recycle the id at the front of the released region. Its `sparse`
            // entry already points at this slot, so growing the retained
            // region by one is all that is needed to mark it retained.
            self.dense[self.live_count]
        } else {
            // Allocate a brand-new id, growing both lists in lock-step. The
            // new id lands at index `live_count`, and its own value is that
            // index, so `sparse` records the id's backing integer as its
            // position.
            let id = TId::from_usize_id(self.sparse.end());
            self.sparse.push(id.to_backing());
            self.dense.push(id);
            id
        };

        self.live_count += 1;

        id
    }

    /// Decodes a value stored in `sparse` back into a `usize` index.
    fn index_of(backing: TId::Backing) -> usize {
        TId::from_backing(backing).to_usize_id().to_usize()
    }
}

impl<TId: Id> Default for IdStruct<TId> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, TId: Id> IntoIterator for &'a IdStruct<TId> {
    type Item = TId;
    type IntoIter = IdStructIter<'a, TId>;

    fn into_iter(self) -> Self::IntoIter {
        IdStructIter::from_live(&self.dense[..self.live_count])
    }
}
