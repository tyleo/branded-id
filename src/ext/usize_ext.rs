use crate::{internal::Sealed, UsizeId};

pub trait UsizeExt: Sealed {
    fn to_usize_id<TMarker>(self) -> UsizeId<TMarker>;
}

impl UsizeExt for usize {
    fn to_usize_id<TMarker>(self) -> UsizeId<TMarker> {
        UsizeId::from_usize(self)
    }
}
