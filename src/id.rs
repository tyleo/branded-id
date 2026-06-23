use crate::{I32Id, IsizeId, U32Id, UsizeId};

/// A typed integer id that can index the columnar storage behind an
/// [`IdStruct`](crate::soa::IdStruct) and its
/// [`IdField`](crate::soa::IdField)s.
///
/// Every id type in this crate implements `Id`, so the id-pool and field
/// machinery works the same regardless of the integer width chosen for ids.
/// [`UsizeId`] is the canonical form because it indexes a [`Vec`] directly;
/// the other widths convert through it.
pub trait Id: Copy + Eq {
    /// Brand that ties this id to a specific id domain.
    type Brand: ?Sized;

    /// Reconstructs an id from its canonical [`UsizeId`] form.
    fn from_usize_id(id: UsizeId<Self::Brand>) -> Self;

    /// Converts this id to its canonical [`UsizeId`] form, used to index the
    /// backing storage.
    fn to_usize_id(self) -> UsizeId<Self::Brand>;
}

impl<TBrand: ?Sized> Id for I32Id<TBrand> {
    type Brand = TBrand;

    fn from_usize_id(id: UsizeId<TBrand>) -> Self {
        id.to_i32_id()
    }

    fn to_usize_id(self) -> UsizeId<TBrand> {
        UsizeId::from_usize(self.to_i32() as usize)
    }
}

impl<TBrand: ?Sized> Id for IsizeId<TBrand> {
    type Brand = TBrand;

    fn from_usize_id(id: UsizeId<TBrand>) -> Self {
        id.to_isize_id()
    }

    fn to_usize_id(self) -> UsizeId<TBrand> {
        UsizeId::from_usize(self.to_isize() as usize)
    }
}

impl<TBrand: ?Sized> Id for U32Id<TBrand> {
    type Brand = TBrand;

    fn from_usize_id(id: UsizeId<TBrand>) -> Self {
        id.to_u32_id()
    }

    fn to_usize_id(self) -> UsizeId<TBrand> {
        UsizeId::from_usize(self.to_u32() as usize)
    }
}

impl<TBrand: ?Sized> Id for UsizeId<TBrand> {
    type Brand = TBrand;

    fn from_usize_id(id: UsizeId<TBrand>) -> Self {
        id
    }

    fn to_usize_id(self) -> UsizeId<TBrand> {
        self
    }
}
