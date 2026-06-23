use crate::{I32Id, U32Id, UsizeId, internal};

internal::scalar_id! { IsizeId, isize, from_isize, to_isize }

impl<TBrand: ?Sized> IsizeId<TBrand> {
    pub const fn offset(self, value: isize) -> IsizeId<TBrand> {
        IsizeId::from_isize(self.to_isize() + value)
    }

    pub const fn to_i32_id(self) -> I32Id<TBrand> {
        I32Id::from_i32(self.to_isize() as i32)
    }

    pub const fn to_u32_id(self) -> U32Id<TBrand> {
        U32Id::from_u32(self.to_isize() as u32)
    }

    pub const fn to_usize_id(self) -> UsizeId<TBrand> {
        UsizeId::from_usize(self.to_isize() as usize)
    }
}
