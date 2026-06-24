use crate::{I128Id, internal::Sealed};

/// Brand-typed [`I128Id`] conversion for `i128`.
pub trait I128Ext: Sealed {
    /// Converts the `i128` to a brand-typed [`I128Id`].
    fn to_i128_id<TBrand: ?Sized>(self) -> I128Id<TBrand>;
}

impl I128Ext for i128 {
    fn to_i128_id<TBrand: ?Sized>(self) -> I128Id<TBrand> {
        I128Id::from_i128(self)
    }
}
