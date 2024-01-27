use crate::IdPtr;

impl<TMarker, TValue> IdPtr<TMarker, TValue> {
    pub const fn downcast_to<TExtendedMarker>(self) -> IdPtr<TExtendedMarker, TValue>
    where
        TExtendedMarker: crate::Extends<TMarker>,
    {
        IdPtr::from_ptr(self.to_ptr())
    }

    pub const fn upcast_to<TExtendedMarker>(self) -> IdPtr<TExtendedMarker, TValue>
    where
        TMarker: crate::Extends<TExtendedMarker>,
    {
        IdPtr::from_ptr(self.to_ptr())
    }
}
