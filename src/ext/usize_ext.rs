use crate::{UsizeId, internal::Sealed};

/// Brand-typed [`UsizeId`] conversion for `usize`.
pub trait UsizeExt: Sealed {
    /// Converts the `usize` to a brand-typed [`UsizeId`].
    fn to_usize_id<TBrand: ?Sized>(self) -> UsizeId<TBrand>;
}

impl UsizeExt for usize {
    fn to_usize_id<TBrand: ?Sized>(self) -> UsizeId<TBrand> {
        UsizeId::from_usize(self)
    }
}
