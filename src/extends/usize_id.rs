use crate::{UsizeId, extends::Extends};

impl<TBrand: ?Sized> UsizeId<TBrand> {
    pub const fn downcast_to<TExtendedBrand>(self) -> UsizeId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        UsizeId::from_usize(self.to_usize())
    }

    pub const fn upcast_to<TExtendedBrand>(self) -> UsizeId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        UsizeId::from_usize(self.to_usize())
    }
}
