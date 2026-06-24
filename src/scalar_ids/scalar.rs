use crate::{
    I8Id, I16Id, I32Id, I64Id, I128Id, Id, IsizeId, U8Id, U16Id, U32Id, U64Id, U128Id, UsizeId,
    internal::Sealed,
};

/// The primitive integer that backs a branded id, paired with the id type that
/// wraps it.
///
/// This is the reverse of [`Id`]: where [`Id`] maps an id type to its brand and
/// canonical [`UsizeId`], `Scalar` maps a primitive integer to the
/// [`Id`](Self::Id) that wraps it. The `soa` `IdStruct` pool uses it to be
/// keyed by a brand alone while choosing its integer width separately: an
/// `IdStruct<BFoo>` stores indices as `u32` and hands out [`U32Id`], while
/// `IdStruct<BFoo, usize>` stores them as `usize` and hands out [`UsizeId`].
pub trait Scalar: Copy + Sealed {
    /// The branded id wrapping this integer for `TBrand` (e.g. [`U32Id`] for
    /// `u32`).
    type Id<TBrand: ?Sized>: Id<Brand = TBrand>;

    /// Reconstructs the integer from a `usize` index.
    fn from_usize(index: usize) -> Self;

    /// Widens the integer back to a `usize` index.
    fn to_usize(self) -> usize;
}

impl Scalar for i8 {
    type Id<TBrand: ?Sized> = I8Id<TBrand>;

    fn from_usize(index: usize) -> Self {
        index as i8
    }

    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Scalar for i16 {
    type Id<TBrand: ?Sized> = I16Id<TBrand>;

    fn from_usize(index: usize) -> Self {
        index as i16
    }

    fn to_usize(self) -> usize {
        self as usize
    }
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

impl Scalar for i64 {
    type Id<TBrand: ?Sized> = I64Id<TBrand>;

    fn from_usize(index: usize) -> Self {
        index as i64
    }

    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Scalar for i128 {
    type Id<TBrand: ?Sized> = I128Id<TBrand>;

    fn from_usize(index: usize) -> Self {
        index as i128
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

impl Scalar for u8 {
    type Id<TBrand: ?Sized> = U8Id<TBrand>;

    fn from_usize(index: usize) -> Self {
        index as u8
    }

    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Scalar for u16 {
    type Id<TBrand: ?Sized> = U16Id<TBrand>;

    fn from_usize(index: usize) -> Self {
        index as u16
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

impl Scalar for u64 {
    type Id<TBrand: ?Sized> = U64Id<TBrand>;

    fn from_usize(index: usize) -> Self {
        index as u64
    }

    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Scalar for u128 {
    type Id<TBrand: ?Sized> = U128Id<TBrand>;

    fn from_usize(index: usize) -> Self {
        index as u128
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
