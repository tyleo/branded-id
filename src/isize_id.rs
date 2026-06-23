use crate::{I32Id, U32Id, UsizeId, internal};

internal::scalar_id! { IsizeId, isize, from_isize, to_isize }

impl<TBrand: ?Sized> IsizeId<TBrand> {
    /// Returns the id advanced by `value`, keeping the brand.
    ///
    /// # Panics
    /// Panics if the addition overflows `isize`.
    pub const fn offset(self, value: isize) -> IsizeId<TBrand> {
        IsizeId::from_isize(self.to_isize() + value)
    }

    /// Reinterprets the id as an [`I32Id`] with an `as` cast, which can
    /// truncate or change sign.
    pub const fn to_i32_id(self) -> I32Id<TBrand> {
        I32Id::from_i32(self.to_isize() as i32)
    }

    /// Reinterprets the id as a [`U32Id`] with an `as` cast.
    pub const fn to_u32_id(self) -> U32Id<TBrand> {
        U32Id::from_u32(self.to_isize() as u32)
    }

    /// Reinterprets the id as a [`UsizeId`] with an `as` cast.
    pub const fn to_usize_id(self) -> UsizeId<TBrand> {
        UsizeId::from_usize(self.to_isize() as usize)
    }
}
