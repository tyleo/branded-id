use crate::UsizeId;

impl<TMarker> UsizeId<TMarker> {
    pub const fn downcast_to<TExtendedMarker>(self) -> UsizeId<TExtendedMarker>
    where
        TExtendedMarker: crate::Extends<TMarker>,
    {
        UsizeId::from_usize(self.to_usize())
    }

    pub const fn upcast_to<TExtendedMarker>(self) -> UsizeId<TExtendedMarker>
    where
        TMarker: crate::Extends<TExtendedMarker>,
    {
        UsizeId::from_usize(self.to_usize())
    }
}
