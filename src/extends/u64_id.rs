use crate::{U64Id, extends::Extends};

impl<TBrand: ?Sized> U64Id<TBrand> {
    /// Returns the same index rebranded toward the extending brand.
    pub const fn downcast_to<TExtendedBrand>(self) -> U64Id<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        U64Id::from_u64(self.to_u64())
    }

    /// Returns the same index rebranded toward the base brand.
    pub const fn upcast_to<TExtendedBrand>(self) -> U64Id<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        U64Id::from_u64(self.to_u64())
    }
}
