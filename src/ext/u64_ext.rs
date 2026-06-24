use crate::{U64Id, internal::Sealed};

/// Brand-typed [`U64Id`] conversion for `u64`.
pub trait U64Ext: Sealed {
    /// Converts the `u64` to a brand-typed [`U64Id`].
    fn to_u64_id<TBrand: ?Sized>(self) -> U64Id<TBrand>;
}

impl U64Ext for u64 {
    fn to_u64_id<TBrand: ?Sized>(self) -> U64Id<TBrand> {
        U64Id::from_u64(self)
    }
}
