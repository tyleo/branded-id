use crate::{IdPtr, internal::Sealed};

/// Brand-typed [`IdPtr`] conversion for `*const TValue`.
pub trait PtrExt<TValue>: Sealed {
    /// Wraps the raw pointer in a brand-typed [`IdPtr`].
    fn to_id_ptr<TBrand: ?Sized>(self) -> IdPtr<TBrand, TValue>;
}

impl<TValue> PtrExt<TValue> for *const TValue {
    fn to_id_ptr<TBrand: ?Sized>(self) -> IdPtr<TBrand, TValue> {
        IdPtr::from_ptr(self)
    }
}
