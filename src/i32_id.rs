use crate::{IsizeId, U32Id, UsizeId, internal};

internal::scalar_id! { I32Id, i32, from_i32, to_i32 }

impl<TMarker: ?Sized> I32Id<TMarker> {
    pub const fn to_isize_id(self) -> IsizeId<TMarker> {
        IsizeId::from_isize(self.to_i32() as isize)
    }

    pub const fn to_u32_id(self) -> U32Id<TMarker> {
        U32Id::from_u32(self.to_i32() as u32)
    }

    pub const fn to_usize_id(self) -> UsizeId<TMarker> {
        UsizeId::from_usize(self.to_i32() as usize)
    }
}
