use crate::{IdPtr, extends::Extends};

impl<TBrand: ?Sized, TValue: ?Sized> IdPtr<TBrand, TValue> {
    pub const fn downcast_to<TExtendedBrand>(self) -> IdPtr<TExtendedBrand, TValue>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdPtr::from_ptr(self.to_ptr())
    }

    pub const fn upcast_to<TExtendedBrand>(self) -> IdPtr<TExtendedBrand, TValue>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdPtr::from_ptr(self.to_ptr())
    }
}
