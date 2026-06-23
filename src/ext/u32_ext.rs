use crate::{U32Id, internal::Sealed};

pub trait U32Ext: Sealed {
    fn to_u32_id<TMarker: ?Sized>(self) -> U32Id<TMarker>;
}

impl U32Ext for u32 {
    fn to_u32_id<TMarker: ?Sized>(self) -> U32Id<TMarker> {
        U32Id::from_u32(self)
    }
}
