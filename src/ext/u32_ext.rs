use crate::{U32Id, internal::Sealed};

/// Brand-typed [`U32Id`] conversion for `u32`.
pub trait U32Ext: Sealed {
    /// Converts the `u32` to a brand-typed [`U32Id`].
    fn to_u32_id<TBrand: ?Sized>(self) -> U32Id<TBrand>;
}

impl U32Ext for u32 {
    fn to_u32_id<TBrand: ?Sized>(self) -> U32Id<TBrand> {
        U32Id::from_u32(self)
    }
}
