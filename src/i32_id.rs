use crate::{IsizeId, U32Id, UsizeId, internal};

internal::scalar_id! { I32Id, i32, from_i32, to_i32 }

impl<TBrand: ?Sized> I32Id<TBrand> {
    /// Reinterprets the id as an [`IsizeId`] with an `as` cast, which can
    /// truncate or change sign.
    pub const fn to_isize_id(self) -> IsizeId<TBrand> {
        IsizeId::from_isize(self.to_i32() as isize)
    }

    /// Reinterprets the id as a [`U32Id`] with an `as` cast.
    pub const fn to_u32_id(self) -> U32Id<TBrand> {
        U32Id::from_u32(self.to_i32() as u32)
    }

    /// Reinterprets the id as a [`UsizeId`] with an `as` cast.
    pub const fn to_usize_id(self) -> UsizeId<TBrand> {
        UsizeId::from_usize(self.to_i32() as usize)
    }
}
