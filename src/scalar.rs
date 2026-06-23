use crate::{I32Id, Id, IsizeId, U32Id, UsizeId, internal::Sealed};

/// The primitive integer that backs a branded id, paired with the id type that
/// wraps it.
///
/// This is the reverse of [`Id`]: where [`Id`] maps an id type to its brand and
/// canonical [`UsizeId`], `Scalar` maps a primitive integer (`u32`, `usize`,
/// `i32`, `isize`) to the [`Id`](Self::Id) that wraps it. The `soa` `IdStruct`
/// pool uses it to be keyed by a brand alone while choosing its integer width
/// separately: an `IdStruct<BFoo>` stores indices as `u32` and hands out
/// [`U32Id`], while `IdStruct<BFoo, usize>` stores them as `usize` and hands
/// out [`UsizeId`].
pub trait Scalar: Copy + Sealed {
    /// The branded id wrapping this integer for `TBrand` (e.g. [`U32Id`] for
    /// `u32`).
    type Id<TBrand: ?Sized>: Id<Brand = TBrand>;

    /// Reconstructs the integer from a `usize` index.
    fn from_usize(index: usize) -> Self;

    /// Widens the integer back to a `usize` index.
    fn to_usize(self) -> usize;
}

impl Scalar for i32 {
    type Id<TBrand: ?Sized> = I32Id<TBrand>;

    fn from_usize(index: usize) -> Self {
        index as i32
    }

    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Scalar for isize {
    type Id<TBrand: ?Sized> = IsizeId<TBrand>;

    fn from_usize(index: usize) -> Self {
        index as isize
    }

    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Scalar for u32 {
    type Id<TBrand: ?Sized> = U32Id<TBrand>;

    fn from_usize(index: usize) -> Self {
        index as u32
    }

    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Scalar for usize {
    type Id<TBrand: ?Sized> = UsizeId<TBrand>;

    fn from_usize(index: usize) -> Self {
        index
    }

    fn to_usize(self) -> usize {
        self
    }
}
