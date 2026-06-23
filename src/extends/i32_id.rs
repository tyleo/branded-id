use crate::{I32Id, extends::Extends};

impl<TMarker: ?Sized> I32Id<TMarker> {
    pub const fn downcast_to<TExtendedMarker>(self) -> I32Id<TExtendedMarker>
    where
        TExtendedMarker: Extends<TMarker> + ?Sized,
    {
        I32Id::from_i32(self.to_i32())
    }

    pub const fn upcast_to<TExtendedMarker>(self) -> I32Id<TExtendedMarker>
    where
        TExtendedMarker: ?Sized,
        TMarker: Extends<TExtendedMarker>,
    {
        I32Id::from_i32(self.to_i32())
    }
}
