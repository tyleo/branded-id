use crate::{U32Id, extends::Extends};

impl<TMarker: ?Sized> U32Id<TMarker> {
    pub const fn downcast_to<TExtendedMarker>(self) -> U32Id<TExtendedMarker>
    where
        TExtendedMarker: Extends<TMarker> + ?Sized,
    {
        U32Id::from_u32(self.to_u32())
    }

    pub const fn upcast_to<TExtendedMarker>(self) -> U32Id<TExtendedMarker>
    where
        TExtendedMarker: ?Sized,
        TMarker: Extends<TExtendedMarker>,
    {
        U32Id::from_u32(self.to_u32())
    }
}
