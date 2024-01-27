use crate::MutIdPtr;

impl<TMarker, TValue> MutIdPtr<TMarker, TValue> {
    pub const fn downcast_to<TExtendedMarker>(self) -> MutIdPtr<TExtendedMarker, TValue>
    where
        TExtendedMarker: crate::Extends<TMarker>,
    {
        MutIdPtr::from_mut_ptr(self.to_mut_ptr())
    }

    pub const fn upcast_to<TExtendedMarker>(self) -> MutIdPtr<TExtendedMarker, TValue>
    where
        TMarker: crate::Extends<TExtendedMarker>,
    {
        MutIdPtr::from_mut_ptr(self.to_mut_ptr())
    }
}
