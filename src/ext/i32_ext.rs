use crate::{I32Id, internal::Sealed};

pub trait I32Ext: Sealed {
    fn to_i32_id<TMarker: ?Sized>(self) -> I32Id<TMarker>;
}

impl I32Ext for i32 {
    fn to_i32_id<TMarker: ?Sized>(self) -> I32Id<TMarker> {
        I32Id::from_i32(self)
    }
}
