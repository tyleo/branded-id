use crate::{IdArray, extends::Extends};

impl<TBrand: ?Sized, TValue, const N: usize> IdArray<TBrand, TValue, N> {
    /// Reborrows toward the extending brand.
    pub const fn downcast_as<TExtendedBrand>(&self) -> &IdArray<TExtendedBrand, TValue, N>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdArray::from_array_ref(self.as_array())
    }

    /// Reborrows toward the base brand.
    pub const fn upcast_as<TExtendedBrand>(&self) -> &IdArray<TExtendedBrand, TValue, N>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdArray::from_array_ref(self.as_array())
    }

    /// Mutably reborrows toward the extending brand.
    pub fn downcast_as_mut<TExtendedBrand>(&mut self) -> &mut IdArray<TExtendedBrand, TValue, N>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdArray::from_mut_array(self.as_mut_array())
    }

    /// Mutably reborrows toward the base brand.
    pub fn upcast_as_mut<TExtendedBrand>(&mut self) -> &mut IdArray<TExtendedBrand, TValue, N>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdArray::from_mut_array(self.as_mut_array())
    }

    /// Consumes and rebrands toward the extending brand.
    pub fn downcast_into<TExtendedBrand>(self) -> IdArray<TExtendedBrand, TValue, N>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdArray::from_array(self.into_array())
    }

    /// Consumes and rebrands toward the base brand.
    pub fn upcast_into<TExtendedBrand>(self) -> IdArray<TExtendedBrand, TValue, N>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdArray::from_array(self.into_array())
    }
}
