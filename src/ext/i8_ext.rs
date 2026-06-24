use crate::{I8Id, internal::Sealed};

/// Brand-typed [`I8Id`] conversion for `i8`.
pub trait I8Ext: Sealed {
    /// Converts the `i8` to a brand-typed [`I8Id`].
    fn to_i8_id<TBrand: ?Sized>(self) -> I8Id<TBrand>;
}

impl I8Ext for i8 {
    fn to_i8_id<TBrand: ?Sized>(self) -> I8Id<TBrand> {
        I8Id::from_i8(self)
    }
}
