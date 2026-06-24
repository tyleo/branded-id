use crate::{I64Id, internal::Sealed};

/// Brand-typed [`I64Id`] conversion for `i64`.
pub trait I64Ext: Sealed {
    /// Converts the `i64` to a brand-typed [`I64Id`].
    fn to_i64_id<TBrand: ?Sized>(self) -> I64Id<TBrand>;
}

impl I64Ext for i64 {
    fn to_i64_id<TBrand: ?Sized>(self) -> I64Id<TBrand> {
        I64Id::from_i64(self)
    }
}
