use crate::{U16Id, extends::Extends};

impl<TBrand: ?Sized> U16Id<TBrand> {
    /// Returns the same index rebranded toward the extending brand.
    pub const fn downcast_to<TExtendedBrand>(self) -> U16Id<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        U16Id::from_u16(self.to_u16())
    }

    /// Returns the same index rebranded toward the base brand.
    pub const fn upcast_to<TExtendedBrand>(self) -> U16Id<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        U16Id::from_u16(self.to_u16())
    }
}
