use crate::{I8Id, extends::Extends};

impl<TBrand: ?Sized> I8Id<TBrand> {
    /// Returns the same index rebranded toward the extending brand.
    pub const fn downcast_to<TExtendedBrand>(self) -> I8Id<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        I8Id::from_i8(self.to_i8())
    }

    /// Returns the same index rebranded toward the base brand.
    pub const fn upcast_to<TExtendedBrand>(self) -> I8Id<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        I8Id::from_i8(self.to_i8())
    }
}
