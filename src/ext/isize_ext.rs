use crate::{IsizeId, internal::Sealed};

pub trait IsizeExt: Sealed {
    fn to_isize_id<TMarker: ?Sized>(self) -> IsizeId<TMarker>;
}

impl IsizeExt for isize {
    fn to_isize_id<TMarker: ?Sized>(self) -> IsizeId<TMarker> {
        IsizeId::from_isize(self)
    }
}
