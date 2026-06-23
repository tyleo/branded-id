use crate::{MutIdPtr, internal::Sealed};

/// Brand-typed [`MutIdPtr`] conversion for `*mut TValue`.
pub trait MutPtrExt<TValue>: Sealed {
    /// Wraps the raw pointer in a brand-typed [`MutIdPtr`].
    fn to_mut_id_ptr<TBrand: ?Sized>(self) -> MutIdPtr<TBrand, TValue>;
}

impl<TValue> MutPtrExt<TValue> for *mut TValue {
    fn to_mut_id_ptr<TBrand: ?Sized>(self) -> MutIdPtr<TBrand, TValue> {
        MutIdPtr::from_mut_ptr(self)
    }
}
