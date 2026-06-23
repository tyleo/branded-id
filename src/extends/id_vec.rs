use crate::{IdVec, extends::Extends};

impl<TBrand: ?Sized, TValue> IdVec<TBrand, TValue> {
    /// Reborrows toward the extending brand.
    pub const fn downcast_as<TExtendedBrand>(&self) -> &IdVec<TExtendedBrand, TValue>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdVec::from_vec_ref(self.as_vec())
    }

    /// Reborrows toward the base brand.
    pub const fn upcast_as<TExtendedBrand>(&self) -> &IdVec<TExtendedBrand, TValue>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdVec::from_vec_ref(self.as_vec())
    }

    /// Mutably reborrows toward the extending brand.
    pub fn downcast_as_mut<TExtendedBrand>(&mut self) -> &mut IdVec<TExtendedBrand, TValue>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdVec::from_mut_vec(self.as_mut_vec())
    }

    /// Mutably reborrows toward the base brand.
    pub fn upcast_as_mut<TExtendedBrand>(&mut self) -> &mut IdVec<TExtendedBrand, TValue>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdVec::from_mut_vec(self.as_mut_vec())
    }

    /// Consumes and rebrands toward the extending brand.
    pub fn downcast_into<TExtendedBrand>(self) -> IdVec<TExtendedBrand, TValue>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IdVec::from_vec(self.into_vec())
    }

    /// Consumes and rebrands toward the base brand.
    pub fn upcast_into<TExtendedBrand>(self) -> IdVec<TExtendedBrand, TValue>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IdVec::from_vec(self.into_vec())
    }
}
