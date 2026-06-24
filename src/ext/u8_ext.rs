use crate::{U8Id, internal::Sealed};

/// Brand-typed [`U8Id`] conversion for `u8`.
pub trait U8Ext: Sealed {
    /// Converts the `u8` to a brand-typed [`U8Id`].
    fn to_u8_id<TBrand: ?Sized>(self) -> U8Id<TBrand>;
}

impl U8Ext for u8 {
    fn to_u8_id<TBrand: ?Sized>(self) -> U8Id<TBrand> {
        U8Id::from_u8(self)
    }
}
