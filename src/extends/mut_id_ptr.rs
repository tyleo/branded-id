use crate::{MutIdPtr, extends::Extends};

impl<TMarker: ?Sized, TValue: ?Sized> MutIdPtr<TMarker, TValue> {
    pub const fn downcast_to<TExtendedMarker>(self) -> MutIdPtr<TExtendedMarker, TValue>
    where
        TExtendedMarker: Extends<TMarker> + ?Sized,
    {
        MutIdPtr::from_mut_ptr(self.to_mut_ptr())
    }

    pub const fn upcast_to<TExtendedMarker>(self) -> MutIdPtr<TExtendedMarker, TValue>
    where
        TExtendedMarker: ?Sized,
        TMarker: Extends<TExtendedMarker>,
    {
        MutIdPtr::from_mut_ptr(self.to_mut_ptr())
    }
}
