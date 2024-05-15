use crate::{extends::Extends, IsizeId};

impl<TMarker> IsizeId<TMarker> {
    pub const fn downcast_to<TExtendedMarker>(self) -> IsizeId<TExtendedMarker>
    where
        TExtendedMarker: Extends<TMarker>,
    {
        IsizeId::from_isize(self.to_isize())
    }

    pub const fn upcast_to<TExtendedMarker>(self) -> IsizeId<TExtendedMarker>
    where
        TMarker: Extends<TExtendedMarker>,
    {
        IsizeId::from_isize(self.to_isize())
    }
}
