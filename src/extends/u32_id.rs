use crate::U32Id;

impl<TMarker> U32Id<TMarker> {
    pub const fn downcast_to<TExtendedMarker>(self) -> U32Id<TExtendedMarker>
    where
        TExtendedMarker: crate::Extends<TMarker>,
    {
        U32Id::from_u32(self.to_u32())
    }

    pub const fn upcast_to<TExtendedMarker>(self) -> U32Id<TExtendedMarker>
    where
        TMarker: crate::Extends<TExtendedMarker>,
    {
        U32Id::from_u32(self.to_u32())
    }
}
