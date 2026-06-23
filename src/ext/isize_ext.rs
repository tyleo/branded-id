use crate::{IsizeId, internal::Sealed};

/// Brand-typed [`IsizeId`] conversion for `isize`.
pub trait IsizeExt: Sealed {
    /// Converts the `isize` to a brand-typed [`IsizeId`].
    fn to_isize_id<TBrand: ?Sized>(self) -> IsizeId<TBrand>;
}

impl IsizeExt for isize {
    fn to_isize_id<TBrand: ?Sized>(self) -> IsizeId<TBrand> {
        IsizeId::from_isize(self)
    }
}
