use crate::{internal::Sealed, IsizeId};

pub trait IsizeExt: Sealed {
    fn to_isize_id<TMarker>(self) -> IsizeId<TMarker>;
}

impl IsizeExt for isize {
    fn to_isize_id<TMarker>(self) -> IsizeId<TMarker> {
        IsizeId::from_isize(self)
    }
}
