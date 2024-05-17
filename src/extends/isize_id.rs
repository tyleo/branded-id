use crate::{extends::Extends, IsizeId};

impl<TMarker: ?Sized> IsizeId<TMarker> {
    pub const fn downcast_to<TExtendedMarker: ?Sized>(self) -> IsizeId<TExtendedMarker>
    where
        TExtendedMarker: Extends<TMarker>,
    {
        IsizeId::from_isize(self.to_isize())
    }

    pub const fn upcast_to<TExtendedMarker: ?Sized>(self) -> IsizeId<TExtendedMarker>
    where
        TMarker: Extends<TExtendedMarker>,
    {
        IsizeId::from_isize(self.to_isize())
    }
}
