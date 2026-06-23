use crate::{IdPtr, extends::Extends};

impl<TBrand: ?Sized, TValue: ?Sized> IdPtr<TBrand, TValue> {
    /// Returns the same pointer rebranded toward the extending brand.
    pub const fn downcast_to<TExtendedBrand>(self) -> IdPtr<TExtendedBrand, TValue>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdPtr::from_ptr(self.to_ptr())
    }

    /// Returns the same pointer rebranded toward the base brand.
    pub const fn upcast_to<TExtendedBrand>(self) -> IdPtr<TExtendedBrand, TValue>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdPtr::from_ptr(self.to_ptr())
    }
}
