use crate::{I32Id, U32Id, UsizeId, internal};

internal::scalar_id! { IsizeId, isize, from_isize, to_isize }

impl<TMarker: ?Sized> IsizeId<TMarker> {
    pub const fn offset(self, value: isize) -> IsizeId<TMarker> {
        IsizeId::from_isize(self.to_isize() + value)
    }

    pub const fn to_i32_id(self) -> I32Id<TMarker> {
        I32Id::from_i32(self.to_isize() as i32)
    }

    pub const fn to_u32_id(self) -> U32Id<TMarker> {
        U32Id::from_u32(self.to_isize() as u32)
    }

    pub const fn to_usize_id(self) -> UsizeId<TMarker> {
        UsizeId::from_usize(self.to_isize() as usize)
    }
}
