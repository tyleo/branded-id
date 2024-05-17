use crate::{extends::Extends, U32Id};

impl<TMarker: ?Sized> U32Id<TMarker> {
    pub const fn downcast_to<TExtendedMarker: ?Sized>(self) -> U32Id<TExtendedMarker>
    where
        TExtendedMarker: Extends<TMarker>,
    {
        U32Id::from_u32(self.to_u32())
    }

    pub const fn upcast_to<TExtendedMarker: ?Sized>(self) -> U32Id<TExtendedMarker>
    where
        TMarker: Extends<TExtendedMarker>,
    {
        U32Id::from_u32(self.to_u32())
    }
}
