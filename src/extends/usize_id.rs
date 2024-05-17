use crate::{extends::Extends, UsizeId};

impl<TMarker: ?Sized> UsizeId<TMarker> {
    pub const fn downcast_to<TExtendedMarker: ?Sized>(self) -> UsizeId<TExtendedMarker>
    where
        TExtendedMarker: Extends<TMarker>,
    {
        UsizeId::from_usize(self.to_usize())
    }

    pub const fn upcast_to<TExtendedMarker: ?Sized>(self) -> UsizeId<TExtendedMarker>
    where
        TMarker: Extends<TExtendedMarker>,
    {
        UsizeId::from_usize(self.to_usize())
    }
}
