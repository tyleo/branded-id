use crate::{IdPtr, extends::Extends};

impl<TMarker: ?Sized, TValue: ?Sized> IdPtr<TMarker, TValue> {
    pub const fn downcast_to<TExtendedMarker>(self) -> IdPtr<TExtendedMarker, TValue>
    where
        TExtendedMarker: Extends<TMarker> + ?Sized,
    {
        IdPtr::from_ptr(self.to_ptr())
    }

    pub const fn upcast_to<TExtendedMarker>(self) -> IdPtr<TExtendedMarker, TValue>
    where
        TExtendedMarker: ?Sized,
        TMarker: Extends<TExtendedMarker>,
    {
        IdPtr::from_ptr(self.to_ptr())
    }
}
