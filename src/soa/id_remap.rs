use crate::{Id, IdVec, Scalar};
use std::{
    fmt::{self, Debug},
    hash::{Hash, Hasher},
};

/// The relabeling produced by [`IdStruct::gc`](super::IdStruct::gc).
///
/// `gc` compacts a pool so its live ids become the contiguous range
/// `0..new_len`, and returns this map from each pre-gc id to the id it was
/// relabeled to. Use [`new_id`](Self::new_id) to translate any id stored
/// outside the pool (including ids held *inside* [`IdField`](super::IdField)
/// values), and pass the whole map to [`IdField::gc`](super::IdField::gc) to
/// compact each paired field the same way.
///
/// A released or never-allocated id has no relabeled counterpart;
/// [`new_id`](Self::new_id) reports it as `None`.
pub struct IdRemap<TBrand: ?Sized, TNum: Scalar> {
    /// Indexed by the *old* id: the id it was relabeled to, or `None` if the
    /// old id was not live at gc time (released, or never handed out).
    new_ids: IdVec<TBrand, Option<TNum::Id<TBrand>>>,

    /// The number of live ids after the gc, i.e. the length of the contiguous
    /// `0..new_len` range the live ids were packed into.
    new_len: usize,
}

impl<TBrand: ?Sized, TNum: Scalar> IdRemap<TBrand, TNum> {
    /// Builds a remap from the per-old-id relabeling and the post-gc live
    /// count. The two must agree: `new_len` equals the number of `Some`
    /// entries, which form the bijection onto `0..new_len`.
    pub(crate) fn from_parts(
        new_ids: IdVec<TBrand, Option<TNum::Id<TBrand>>>,
        new_len: usize,
    ) -> Self {
        Self { new_ids, new_len }
    }

    /// The per-old-id relabeling, for [`IdField::gc`](super::IdField::gc) to
    /// scatter each live value from its old slot to its new one.
    pub(crate) fn new_ids(&self) -> &IdVec<TBrand, Option<TNum::Id<TBrand>>> {
        &self.new_ids
    }

    /// Whether the gc left no live ids.
    pub fn is_empty(&self) -> bool {
        self.new_len == 0
    }

    /// The id `old` was relabeled to, or `None` if `old` was not live when the
    /// pool was [`gc`](super::IdStruct::gc)'d (it had been released, or was
    /// never handed out and so sits at or beyond [`old_len`](Self::old_len)).
    pub fn new_id(&self, old: impl Id<Brand = TBrand>) -> Option<TNum::Id<TBrand>> {
        let old = old.to_usize_id().to_usize();
        self.new_ids.as_vec().get(old).copied().flatten()
    }

    /// The number of live ids after the gc: the relabeled ids fill the
    /// contiguous range `0..new_len`.
    pub fn new_len(&self) -> usize {
        self.new_len
    }

    /// The size of the pre-gc id space the map covers, one past the largest id
    /// the pool had ever handed out. [`new_id`](Self::new_id) is `None` for any
    /// id at or above this.
    pub fn old_len(&self) -> usize {
        self.new_ids.len()
    }
}

impl<TBrand: ?Sized, TNum: Scalar> Clone for IdRemap<TBrand, TNum> {
    fn clone(&self) -> Self {
        Self {
            new_ids: self.new_ids.clone(),
            new_len: self.new_len,
        }
    }
}

impl<TBrand: ?Sized, TNum: Scalar> Debug for IdRemap<TBrand, TNum>
where
    TNum::Id<TBrand>: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IdRemap")
            .field("new_ids", &self.new_ids)
            .field("new_len", &self.new_len)
            .finish()
    }
}

impl<TBrand: ?Sized, TNum: Scalar> Default for IdRemap<TBrand, TNum> {
    fn default() -> Self {
        Self::from_parts(IdVec::new(), 0)
    }
}

impl<TBrand: ?Sized, TNum: Scalar> Eq for IdRemap<TBrand, TNum> {}

impl<TBrand: ?Sized, TNum: Scalar> Hash for IdRemap<TBrand, TNum>
where
    TNum::Id<TBrand>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.new_ids.hash(state);
        self.new_len.hash(state);
    }
}

impl<TBrand: ?Sized, TNum: Scalar> PartialEq for IdRemap<TBrand, TNum> {
    fn eq(&self, other: &Self) -> bool {
        // `new_len` is redundant with `new_ids` (it counts its `Some` entries),
        // but comparing it too keeps this in step with `Hash` and is cheap.
        self.new_len == other.new_len && self.new_ids == other.new_ids
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        self.new_len != other.new_len || self.new_ids != other.new_ids
    }
}
