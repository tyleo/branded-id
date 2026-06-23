use crate::{IdVec, Scalar};
use std::{
    fmt::{self, Debug},
    hash::{Hash, Hasher},
};

/// Borrowed views of the internal lists of an [`IdStruct`](super::IdStruct),
/// as returned by [`IdStruct::as_raw_parts`](super::IdStruct::as_raw_parts).
pub struct IdStructRawParts<'a, TBrand: ?Sized, TNum: Scalar> {
    /// The ids currently retained, packed.
    pub live: &'a [TNum::Id<TBrand>],

    /// Per-id: its index in the dense list (`live` followed by `free`),
    /// stored in the pool's integer width `TNum`.
    pub sparse: &'a IdVec<TBrand, TNum>,

    /// Released ids available for reuse; the next id to be recycled is first.
    pub free: &'a [TNum::Id<TBrand>],
}

impl<TBrand: ?Sized, TNum: Scalar> Clone for IdStructRawParts<'_, TBrand, TNum> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TBrand: ?Sized, TNum: Scalar> Copy for IdStructRawParts<'_, TBrand, TNum> {}

impl<TBrand: ?Sized, TNum: Scalar> Debug for IdStructRawParts<'_, TBrand, TNum>
where
    TNum: Debug,
    TNum::Id<TBrand>: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IdStructRawParts")
            .field("live", &self.live)
            .field("sparse", &self.sparse)
            .field("free", &self.free)
            .finish()
    }
}

impl<TBrand: ?Sized, TNum: Scalar> Eq for IdStructRawParts<'_, TBrand, TNum> where TNum: Eq {}

impl<TBrand: ?Sized, TNum: Scalar> Hash for IdStructRawParts<'_, TBrand, TNum>
where
    TNum: Hash,
    TNum::Id<TBrand>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.live.hash(state);
        self.sparse.hash(state);
        self.free.hash(state);
    }
}

impl<TBrand: ?Sized, TNum: Scalar> PartialEq for IdStructRawParts<'_, TBrand, TNum>
where
    TNum: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        // Compares the borrowed contents field by field. Like `IdStruct`'s own
        // equality this is a structural snapshot, so two pools with the same
        // live set but different free-region layout compare unequal.
        self.live == other.live && self.sparse == other.sparse && self.free == other.free
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        self.live != other.live || self.sparse != other.sparse || self.free != other.free
    }
}
