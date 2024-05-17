use crate::{extends::Extends, I32Id};

impl<TMarker: ?Sized> I32Id<TMarker> {
    pub const fn downcast_to<TExtendedMarker: ?Sized>(self) -> I32Id<TExtendedMarker>
    where
        TExtendedMarker: Extends<TMarker>,
    {
        I32Id::from_i32(self.to_i32())
    }

    pub const fn upcast_to<TExtendedMarker: ?Sized>(self) -> I32Id<TExtendedMarker>
    where
        TMarker: Extends<TExtendedMarker>,
    {
        I32Id::from_i32(self.to_i32())
    }
}
