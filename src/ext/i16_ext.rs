use crate::{I16Id, internal::Sealed};

/// Brand-typed [`I16Id`] conversion for `i16`.
pub trait I16Ext: Sealed {
    /// Converts the `i16` to a brand-typed [`I16Id`].
    fn to_i16_id<TBrand: ?Sized>(self) -> I16Id<TBrand>;
}

impl I16Ext for i16 {
    fn to_i16_id<TBrand: ?Sized>(self) -> I16Id<TBrand> {
        I16Id::from_i16(self)
    }
}
