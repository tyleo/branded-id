use crate::{I32Id, IsizeId, UsizeId, internal};

internal::scalar_id! { U32Id, u32, from_u32, to_u32 }

impl<TMarker: ?Sized> U32Id<TMarker> {
    pub const fn to_i32_id(self) -> I32Id<TMarker> {
        I32Id::from_i32(self.to_u32() as i32)
    }

    pub const fn to_isize_id(self) -> IsizeId<TMarker> {
        IsizeId::from_isize(self.to_u32() as isize)
    }

    pub const fn to_usize_id(self) -> UsizeId<TMarker> {
        UsizeId::from_usize(self.to_u32() as usize)
    }
}
