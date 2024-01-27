use crate::IsizeId;

impl<TMarker> IsizeId<TMarker> {
    pub const fn downcast_to<TExtendedMarker>(self) -> IsizeId<TExtendedMarker>
    where
        TExtendedMarker: crate::Extends<TMarker>,
    {
        IsizeId::from_isize(self.to_isize())
    }

    pub const fn upcast_to<TExtendedMarker>(self) -> IsizeId<TExtendedMarker>
    where
        TMarker: crate::Extends<TExtendedMarker>,
    {
        IsizeId::from_isize(self.to_isize())
    }
}
