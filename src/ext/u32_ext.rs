use crate::{internal::Sealed, U32Id};

pub trait U32Ext: Sealed {
    fn to_u32_id<TMarker>(self) -> U32Id<TMarker>;
}

impl U32Ext for u32 {
    fn to_u32_id<TMarker>(self) -> U32Id<TMarker> {
        U32Id::from_u32(self)
    }
}
