use crate::{internal::Sealed, I32Id};

pub trait I32Ext: Sealed {
    fn to_i32_id<TMarker>(self) -> I32Id<TMarker>;
}

impl I32Ext for i32 {
    fn to_i32_id<TMarker>(self) -> I32Id<TMarker> {
        I32Id::from_i32(self)
    }
}
