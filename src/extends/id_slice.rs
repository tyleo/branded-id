use crate::IdSlice;

impl<TMarker, TValue> IdSlice<TMarker, TValue> {
    pub const fn downcast_as<TExtendedMarker>(&self) -> &IdSlice<TExtendedMarker, TValue>
    where
        TExtendedMarker: crate::Extends<TMarker>,
    {
        IdSlice::from_slice(self.as_slice())
    }

    pub const fn upcast_as<TExtendedMarker>(&self) -> &IdSlice<TExtendedMarker, TValue>
    where
        TMarker: crate::Extends<TExtendedMarker>,
    {
        IdSlice::from_slice(self.as_slice())
    }

    pub fn downcast_as_mut<TExtendedMarker>(&mut self) -> &mut IdSlice<TExtendedMarker, TValue>
    where
        TExtendedMarker: crate::Extends<TMarker>,
    {
        IdSlice::from_mut_slice(self.as_mut_slice())
    }

    pub fn upcast_as_mut<TExtendedMarker>(&mut self) -> &mut IdSlice<TExtendedMarker, TValue>
    where
        TMarker: crate::Extends<TExtendedMarker>,
    {
        IdSlice::from_mut_slice(self.as_mut_slice())
    }
}
