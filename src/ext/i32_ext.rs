use crate::{I32Id, internal::Sealed};

/// Brand-typed [`I32Id`] conversion for `i32`.
pub trait I32Ext: Sealed {
    /// Converts the `i32` to a brand-typed [`I32Id`].
    fn to_i32_id<TBrand: ?Sized>(self) -> I32Id<TBrand>;
}

impl I32Ext for i32 {
    fn to_i32_id<TBrand: ?Sized>(self) -> I32Id<TBrand> {
        I32Id::from_i32(self)
    }
}
