use crate::IdVec;

impl<TMarker, TValue> IdVec<TMarker, TValue> {
    pub const fn downcast_as<TExtendedMarker>(&self) -> &IdVec<TExtendedMarker, TValue>
    where
        TExtendedMarker: crate::Extends<TMarker>,
    {
        IdVec::from_vec_ref(self.as_vec())
    }

    pub const fn upcast_as<TExtendedMarker>(&self) -> &IdVec<TExtendedMarker, TValue>
    where
        TMarker: crate::Extends<TExtendedMarker>,
    {
        IdVec::from_vec_ref(self.as_vec())
    }

    pub fn downcast_as_mut<TExtendedMarker>(&mut self) -> &mut IdVec<TExtendedMarker, TValue>
    where
        TExtendedMarker: crate::Extends<TMarker>,
    {
        IdVec::from_mut_vec(self.as_mut_vec())
    }

    pub fn upcast_as_mut<TExtendedMarker>(&mut self) -> &mut IdVec<TExtendedMarker, TValue>
    where
        TMarker: crate::Extends<TExtendedMarker>,
    {
        IdVec::from_mut_vec(self.as_mut_vec())
    }

    pub fn downcast_into<TExtendedMarker>(self) -> IdVec<TExtendedMarker, TValue>
    where
        TExtendedMarker: crate::Extends<TMarker>,
    {
        IdVec::from_vec(self.into_vec())
    }

    pub fn upcast_into<TExtendedMarker>(self) -> IdVec<TExtendedMarker, TValue>
    where
        TMarker: crate::Extends<TExtendedMarker>,
    {
        IdVec::from_vec(self.into_vec())
    }
}
