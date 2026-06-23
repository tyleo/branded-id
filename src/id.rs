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
    /// Marker that ties this id to a specific id domain.
    type Marker: ?Sized;

    /// Reconstructs an id from its canonical [`UsizeId`] form.
    fn from_usize_id(id: UsizeId<Self::Marker>) -> Self;

    /// Converts this id to its canonical [`UsizeId`] form, used to index the
    /// backing storage.
    fn to_usize_id(self) -> UsizeId<Self::Marker>;
}

impl<TMarker: ?Sized> Id for I32Id<TMarker> {
    type Marker = TMarker;

    fn from_usize_id(id: UsizeId<TMarker>) -> Self {
        id.to_i32_id()
    }

    fn to_usize_id(self) -> UsizeId<TMarker> {
        UsizeId::from_usize(self.to_i32() as usize)
    }
}

impl<TMarker: ?Sized> Id for IsizeId<TMarker> {
    type Marker = TMarker;

    fn from_usize_id(id: UsizeId<TMarker>) -> Self {
        id.to_isize_id()
    }

    fn to_usize_id(self) -> UsizeId<TMarker> {
        UsizeId::from_usize(self.to_isize() as usize)
    }
}

impl<TMarker: ?Sized> Id for U32Id<TMarker> {
    type Marker = TMarker;

    fn from_usize_id(id: UsizeId<TMarker>) -> Self {
        id.to_u32_id()
    }

    fn to_usize_id(self) -> UsizeId<TMarker> {
        UsizeId::from_usize(self.to_u32() as usize)
    }
}

impl<TMarker: ?Sized> Id for UsizeId<TMarker> {
    type Marker = TMarker;

    fn from_usize_id(id: UsizeId<TMarker>) -> Self {
        id
    }

    fn to_usize_id(self) -> UsizeId<TMarker> {
        self
    }
}
