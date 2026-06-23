use crate::{MutIdPtr, extends::Extends};

impl<TBrand: ?Sized, TValue: ?Sized> MutIdPtr<TBrand, TValue> {
    pub const fn downcast_to<TExtendedBrand>(self) -> MutIdPtr<TExtendedBrand, TValue>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        MutIdPtr::from_mut_ptr(self.to_mut_ptr())
    }

    pub const fn upcast_to<TExtendedBrand>(self) -> MutIdPtr<TExtendedBrand, TValue>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        MutIdPtr::from_mut_ptr(self.to_mut_ptr())
    }
}
