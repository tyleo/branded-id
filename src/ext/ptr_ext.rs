use crate::{IdPtr, internal::Sealed};

pub trait PtrExt<TValue>: Sealed {
    fn to_id_ptr<TBrand: ?Sized>(self) -> IdPtr<TBrand, TValue>;
}

impl<TValue> PtrExt<TValue> for *const TValue {
    fn to_id_ptr<TBrand: ?Sized>(self) -> IdPtr<TBrand, TValue> {
        IdPtr::from_ptr(self)
    }
}
