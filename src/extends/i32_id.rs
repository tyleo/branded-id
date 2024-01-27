use crate::I32Id;

impl<TMarker> I32Id<TMarker> {
    pub const fn downcast_to<TExtendedMarker>(self) -> I32Id<TExtendedMarker>
    where
        TExtendedMarker: crate::Extends<TMarker>,
    {
        I32Id::from_i32(self.to_i32())
    }

    pub const fn upcast_to<TExtendedMarker>(self) -> I32Id<TExtendedMarker>
    where
        TMarker: crate::Extends<TExtendedMarker>,
    {
        I32Id::from_i32(self.to_i32())
    }
}
