use crate::{IsizeId, extends::Extends};

impl<TBrand: ?Sized> IsizeId<TBrand> {
    /// Returns the same index rebranded toward the extending brand.
    pub const fn downcast_to<TExtendedBrand>(self) -> IsizeId<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        IsizeId::from_isize(self.to_isize())
    }

    /// Returns the same index rebranded toward the base brand.
    pub const fn upcast_to<TExtendedBrand>(self) -> IsizeId<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        IsizeId::from_isize(self.to_isize())
    }
}
