use crate::{I32Id, IsizeId, U32Id, UsizeId, internal::Sealed};

/// A branded integer id of any width, convertible to and from its canonical
/// [`UsizeId`] form.
///
/// Every id type in this crate ([`UsizeId`], [`I32Id`], [`U32Id`],
/// [`IsizeId`]) implements `Id`, so generic code can abstract over the integer
/// width an id uses. [`UsizeId`] is the canonical form because it indexes a
/// [`Vec`] or slice directly; the other widths convert through it.
///
/// The `soa` id pools are one consumer, but `Id` is useful anywhere code needs
/// to be generic over id width. See [`Scalar`](crate::Scalar) for the reverse
/// mapping, from a primitive integer to the id type that wraps it.
pub trait Id: Copy + Eq + Sealed {
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
