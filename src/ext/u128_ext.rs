use crate::{U128Id, internal::Sealed};

/// Brand-typed [`U128Id`] conversion for `u128`.
pub trait U128Ext: Sealed {
    /// Converts the `u128` to a brand-typed [`U128Id`].
    fn to_u128_id<TBrand: ?Sized>(self) -> U128Id<TBrand>;
}

impl U128Ext for u128 {
    fn to_u128_id<TBrand: ?Sized>(self) -> U128Id<TBrand> {
        U128Id::from_u128(self)
    }
}
