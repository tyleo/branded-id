use crate::{I128Id, extends::Extends};

impl<TBrand: ?Sized> I128Id<TBrand> {
    /// Returns the same index rebranded toward the extending brand.
    pub const fn downcast_to<TExtendedBrand>(self) -> I128Id<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        I128Id::from_i128(self.to_i128())
    }

    /// Returns the same index rebranded toward the base brand.
    pub const fn upcast_to<TExtendedBrand>(self) -> I128Id<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        I128Id::from_i128(self.to_i128())
    }
}
