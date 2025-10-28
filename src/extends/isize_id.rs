use crate::{extends::Extends, IsizeId};

impl<TMarker: ?Sized> IsizeId<TMarker> {
    pub const fn downcast_to<TExtendedMarker>(self) -> IsizeId<TExtendedMarker>
    where
        TExtendedMarker: Extends<TMarker> + ?Sized,
    {
        IsizeId::from_isize(self.to_isize())
    }

    pub const fn upcast_to<TExtendedMarker>(self) -> IsizeId<TExtendedMarker>
    where
        TExtendedMarker: ?Sized,
        TMarker: Extends<TExtendedMarker>,
    {
        IsizeId::from_isize(self.to_isize())
    }
}
