use crate::{IdArray, extends::Extends};

impl<TBrand: ?Sized, TValue, const N: usize> IdArray<TBrand, TValue, N> {
    pub const fn downcast_as<TExtendedBrand>(&self) -> &IdArray<TExtendedBrand, TValue, N>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdArray::from_array_ref(self.as_array())
    }

    pub const fn upcast_as<TExtendedBrand>(&self) -> &IdArray<TExtendedBrand, TValue, N>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdArray::from_array_ref(self.as_array())
    }

    pub fn downcast_as_mut<TExtendedBrand>(&mut self) -> &mut IdArray<TExtendedBrand, TValue, N>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdArray::from_mut_array(self.as_mut_array())
    }

    pub fn upcast_as_mut<TExtendedBrand>(&mut self) -> &mut IdArray<TExtendedBrand, TValue, N>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdArray::from_mut_array(self.as_mut_array())
    }

    pub fn downcast_into<TExtendedBrand>(self) -> IdArray<TExtendedBrand, TValue, N>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdArray::from_array(self.into_array())
    }

    pub fn upcast_into<TExtendedBrand>(self) -> IdArray<TExtendedBrand, TValue, N>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdArray::from_array(self.into_array())
    }
}
