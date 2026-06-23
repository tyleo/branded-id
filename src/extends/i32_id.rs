use crate::{I32Id, extends::Extends};

impl<TBrand: ?Sized> I32Id<TBrand> {
    pub const fn downcast_to<TExtendedBrand>(self) -> I32Id<TExtendedBrand>
    where
        TExtendedBrand: Extends<TBrand> + ?Sized,
    {
        I32Id::from_i32(self.to_i32())
    }

    pub const fn upcast_to<TExtendedBrand>(self) -> I32Id<TExtendedBrand>
    where
        TExtendedBrand: ?Sized,
        TBrand: Extends<TExtendedBrand>,
    {
        I32Id::from_i32(self.to_i32())
    }
}
