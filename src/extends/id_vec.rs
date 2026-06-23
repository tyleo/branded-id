use crate::{IdVec, extends::Extends};

impl<TBrand: ?Sized, TValue> IdVec<TBrand, TValue> {
    pub const fn downcast_as<TExtendedBrand>(&self) -> &IdVec<TExtendedBrand, TValue>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdVec::from_vec_ref(self.as_vec())
    }

    pub const fn upcast_as<TExtendedBrand>(&self) -> &IdVec<TExtendedBrand, TValue>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdVec::from_vec_ref(self.as_vec())
    }

    pub fn downcast_as_mut<TExtendedBrand>(&mut self) -> &mut IdVec<TExtendedBrand, TValue>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdVec::from_mut_vec(self.as_mut_vec())
    }

    pub fn upcast_as_mut<TExtendedBrand>(&mut self) -> &mut IdVec<TExtendedBrand, TValue>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdVec::from_mut_vec(self.as_mut_vec())
    }

    pub fn downcast_into<TExtendedBrand>(self) -> IdVec<TExtendedBrand, TValue>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdVec::from_vec(self.into_vec())
    }

    pub fn upcast_into<TExtendedBrand>(self) -> IdVec<TExtendedBrand, TValue>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdVec::from_vec(self.into_vec())
    }
}
