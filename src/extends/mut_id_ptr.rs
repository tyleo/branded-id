use crate::{extends::Extends, MutIdPtr};

impl<TMarker: ?Sized, TValue> MutIdPtr<TMarker, TValue> {
    pub const fn downcast_to<TExtendedMarker: ?Sized>(self) -> MutIdPtr<TExtendedMarker, TValue>
    where
        TExtendedMarker: Extends<TMarker>,
    {
        MutIdPtr::from_mut_ptr(self.to_mut_ptr())
    }

    pub const fn upcast_to<TExtendedMarker: ?Sized>(self) -> MutIdPtr<TExtendedMarker, TValue>
    where
        TMarker: Extends<TExtendedMarker>,
    {
        MutIdPtr::from_mut_ptr(self.to_mut_ptr())
    }
}
