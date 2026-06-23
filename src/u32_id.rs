use crate::{I32Id, IsizeId, UsizeId, internal};

internal::scalar_id! { U32Id, u32, from_u32, to_u32 }

impl<TBrand: ?Sized> U32Id<TBrand> {
    pub const fn to_i32_id(self) -> I32Id<TBrand> {
        I32Id::from_i32(self.to_u32() as i32)
    }

    pub const fn to_isize_id(self) -> IsizeId<TBrand> {
        IsizeId::from_isize(self.to_u32() as isize)
    }

    pub const fn to_usize_id(self) -> UsizeId<TBrand> {
        UsizeId::from_usize(self.to_u32() as usize)
    }
}
