use crate::{U16Id, internal::Sealed};

/// Brand-typed [`U16Id`] conversion for `u16`.
pub trait U16Ext: Sealed {
    /// Converts the `u16` to a brand-typed [`U16Id`].
    fn to_u16_id<TBrand: ?Sized>(self) -> U16Id<TBrand>;
}

impl U16Ext for u16 {
    fn to_u16_id<TBrand: ?Sized>(self) -> U16Id<TBrand> {
        U16Id::from_u16(self)
    }
}
