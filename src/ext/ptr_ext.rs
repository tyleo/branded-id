use crate::{IdPtr, internal::Sealed};

pub trait PtrExt<TValue>: Sealed {
    fn to_id_ptr<TMarker: ?Sized>(self) -> IdPtr<TMarker, TValue>;
}

impl<TValue> PtrExt<TValue> for *const TValue {
    fn to_id_ptr<TMarker: ?Sized>(self) -> IdPtr<TMarker, TValue> {
        IdPtr::from_ptr(self)
    }
}
