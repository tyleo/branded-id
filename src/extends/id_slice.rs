use crate::{IdSlice, extends::Extends};

impl<TBrand: ?Sized, TValue> IdSlice<TBrand, TValue> {
    /// Reborrows toward the extending brand.
    pub const fn downcast_as<TExtendedBrand>(&self) -> &IdSlice<TExtendedBrand, TValue>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdSlice::from_slice(self.as_slice())
    }

    /// Reborrows toward the base brand.
    pub const fn upcast_as<TExtendedBrand>(&self) -> &IdSlice<TExtendedBrand, TValue>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdSlice::from_slice(self.as_slice())
    }

    /// Mutably reborrows toward the extending brand.
    pub fn downcast_as_mut<TExtendedBrand>(&mut self) -> &mut IdSlice<TExtendedBrand, TValue>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdSlice::from_mut_slice(self.as_mut_slice())
    }

    /// Mutably reborrows toward the base brand.
    pub fn upcast_as_mut<TExtendedBrand>(&mut self) -> &mut IdSlice<TExtendedBrand, TValue>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdSlice::from_mut_slice(self.as_mut_slice())
    }
}
