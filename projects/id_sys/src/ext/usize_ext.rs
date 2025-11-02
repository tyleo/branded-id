use crate::{UsizeId, internal::Sealed};

pub trait UsizeExt: Sealed {
    fn to_usize_id<TMarker: ?Sized>(self) -> UsizeId<TMarker>;
}

impl UsizeExt for usize {
    fn to_usize_id<TMarker: ?Sized>(self) -> UsizeId<TMarker> {
        UsizeId::from_usize(self)
    }
}
