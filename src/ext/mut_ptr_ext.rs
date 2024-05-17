use crate::{internal::Sealed, MutIdPtr};

pub trait MutPtrExt<TValue>: Sealed {
    fn to_mut_id_ptr<TMarker: ?Sized>(self) -> MutIdPtr<TMarker, TValue>;
}

impl<TValue> MutPtrExt<TValue> for *mut TValue {
    fn to_mut_id_ptr<TMarker: ?Sized>(self) -> MutIdPtr<TMarker, TValue> {
        MutIdPtr::from_mut_ptr(self)
    }
}
