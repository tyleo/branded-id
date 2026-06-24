use crate::{I64Id, extends::Extends};

impl<TBrand: ?Sized> I64Id<TBrand> {
    /// Returns the same index rebranded toward the extending brand.
    pub const fn downcast_to<TExtendedBrand>(self) -> I64Id<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        I64Id::from_i64(self.to_i64())
    }

    /// Returns the same index rebranded toward the base brand.
    pub const fn upcast_to<TExtendedBrand>(self) -> I64Id<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        I64Id::from_i64(self.to_i64())
    }
}
