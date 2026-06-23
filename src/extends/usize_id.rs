use crate::{UsizeId, extends::Extends};

impl<TMarker: ?Sized> UsizeId<TMarker> {
    pub const fn downcast_to<TExtendedMarker>(self) -> UsizeId<TExtendedMarker>
    where
        TExtendedMarker: Extends<TMarker> + ?Sized,
    {
        UsizeId::from_usize(self.to_usize())
    }

    pub const fn upcast_to<TExtendedMarker>(self) -> UsizeId<TExtendedMarker>
    where
        TExtendedMarker: ?Sized,
        TMarker: Extends<TExtendedMarker>,
    {
        UsizeId::from_usize(self.to_usize())
    }
}
