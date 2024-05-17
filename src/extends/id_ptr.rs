use crate::{extends::Extends, IdPtr};

impl<TMarker: ?Sized, TValue> IdPtr<TMarker, TValue> {
    pub const fn downcast_to<TExtendedMarker: ?Sized>(self) -> IdPtr<TExtendedMarker, TValue>
    where
        TExtendedMarker: Extends<TMarker>,
    {
        IdPtr::from_ptr(self.to_ptr())
    }

    pub const fn upcast_to<TExtendedMarker: ?Sized>(self) -> IdPtr<TExtendedMarker, TValue>
    where
        TMarker: Extends<TExtendedMarker>,
    {
        IdPtr::from_ptr(self.to_ptr())
    }
}
