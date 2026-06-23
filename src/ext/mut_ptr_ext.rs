use crate::{MutIdPtr, internal::Sealed};

pub trait MutPtrExt<TValue>: Sealed {
    fn to_mut_id_ptr<TBrand: ?Sized>(self) -> MutIdPtr<TBrand, TValue>;
}

impl<TValue> MutPtrExt<TValue> for *mut TValue {
    fn to_mut_id_ptr<TBrand: ?Sized>(self) -> MutIdPtr<TBrand, TValue> {
        MutIdPtr::from_mut_ptr(self)
    }
}
