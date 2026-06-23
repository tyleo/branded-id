use crate::{I32Id, IsizeId, U32Id, UsizeId};

/// A branded integer id of any width, convertible to and from its canonical
/// [`UsizeId`] form.
///
/// Every id type in this crate ([`UsizeId`], [`I32Id`], [`U32Id`],
/// [`IsizeId`]) implements `Id`, so generic code can abstract over the integer
/// width an id uses. [`UsizeId`] is the canonical form because it indexes a
/// [`Vec`] or slice directly; the other widths convert through it. Each id also
/// exposes its raw backing integer via [`to_backing`](Self::to_backing).
///
/// The `soa` id pools are one consumer, but `Id` is useful anywhere code needs
/// to be generic over id width.
pub trait Id: Copy + Eq {
    /// Brand that ties this id to a specific id domain.
    type Brand: ?Sized;

    /// The primitive integer this id wraps (e.g. `u32` for [`U32Id`]). Any
    /// index a pool hands out fits in it, so id-keyed storage can record
    /// positions in this width instead of a full `usize`.
    type Backing: Copy;

    /// Reconstructs an id from its canonical [`UsizeId`] form.
    fn from_usize_id(id: UsizeId<Self::Brand>) -> Self;

    /// Converts this id to its canonical [`UsizeId`] form, used to index the
    /// backing storage.
    fn to_usize_id(self) -> UsizeId<Self::Brand>;

    /// Reconstructs an id from its backing integer.
    fn from_backing(backing: Self::Backing) -> Self;

    /// Converts this id to its backing integer.
    fn to_backing(self) -> Self::Backing;
}

impl<TBrand: ?Sized> Id for I32Id<TBrand> {
    type Brand = TBrand;
    type Backing = i32;

    fn from_usize_id(id: UsizeId<TBrand>) -> Self {
        id.to_i32_id()
    }

    fn to_usize_id(self) -> UsizeId<TBrand> {
        UsizeId::from_usize(self.to_i32() as usize)
    }

    fn from_backing(backing: i32) -> Self {
        I32Id::from_i32(backing)
    }

    fn to_backing(self) -> i32 {
        self.to_i32()
    }
}

impl<TBrand: ?Sized> Id for IsizeId<TBrand> {
    type Brand = TBrand;
    type Backing = isize;

    fn from_usize_id(id: UsizeId<TBrand>) -> Self {
        id.to_isize_id()
    }

    fn to_usize_id(self) -> UsizeId<TBrand> {
        UsizeId::from_usize(self.to_isize() as usize)
    }

    fn from_backing(backing: isize) -> Self {
        IsizeId::from_isize(backing)
    }

    fn to_backing(self) -> isize {
        self.to_isize()
    }
}

impl<TBrand: ?Sized> Id for U32Id<TBrand> {
    type Brand = TBrand;
    type Backing = u32;

    fn from_usize_id(id: UsizeId<TBrand>) -> Self {
        id.to_u32_id()
    }

    fn to_usize_id(self) -> UsizeId<TBrand> {
        UsizeId::from_usize(self.to_u32() as usize)
    }

    fn from_backing(backing: u32) -> Self {
        U32Id::from_u32(backing)
    }

    fn to_backing(self) -> u32 {
        self.to_u32()
    }
}

impl<TBrand: ?Sized> Id for UsizeId<TBrand> {
    type Brand = TBrand;
    type Backing = usize;

    fn from_usize_id(id: UsizeId<TBrand>) -> Self {
        id
    }

    fn to_usize_id(self) -> UsizeId<TBrand> {
        self
    }

    fn from_backing(backing: usize) -> Self {
        UsizeId::from_usize(backing)
    }

    fn to_backing(self) -> usize {
        self.to_usize()
    }
}
