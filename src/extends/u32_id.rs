use crate::{U32Id, extends::Extends};

impl<TBrand: ?Sized> U32Id<TBrand> {
    pub const fn downcast_to<TExtendedBrand>(self) -> U32Id<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        U32Id::from_u32(self.to_u32())
    }

    pub const fn upcast_to<TExtendedBrand>(self) -> U32Id<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        U32Id::from_u32(self.to_u32())
    }
}
