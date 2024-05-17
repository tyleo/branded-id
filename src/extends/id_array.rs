use crate::{extends::Extends, IdArray};

impl<TMarker: ?Sized, TValue, const N: usize> IdArray<TMarker, TValue, N> {
    pub const fn downcast_as<TExtendedMarker: ?Sized>(&self) -> &IdArray<TExtendedMarker, TValue, N>
    where
        TExtendedMarker: Extends<TMarker>,
    {
        IdArray::from_array_ref(self.as_array())
    }

    pub const fn upcast_as<TExtendedMarker: ?Sized>(&self) -> &IdArray<TExtendedMarker, TValue, N>
    where
        TMarker: Extends<TExtendedMarker>,
    {
        IdArray::from_array_ref(self.as_array())
    }

    pub fn downcast_as_mut<TExtendedMarker: ?Sized>(
        &mut self,
    ) -> &mut IdArray<TExtendedMarker, TValue, N>
    where
        TExtendedMarker: Extends<TMarker>,
    {
        IdArray::from_mut_array(self.as_mut_array())
    }

    pub fn upcast_as_mut<TExtendedMarker: ?Sized>(
        &mut self,
    ) -> &mut IdArray<TExtendedMarker, TValue, N>
    where
        TMarker: Extends<TExtendedMarker>,
    {
        IdArray::from_mut_array(self.as_mut_array())
    }

    pub fn downcast_into<TExtendedMarker: ?Sized>(self) -> IdArray<TExtendedMarker, TValue, N>
    where
        TExtendedMarker: Extends<TMarker>,
    {
        IdArray::from_array(self.into_array())
    }

    pub fn upcast_into<TExtendedMarker: ?Sized>(self) -> IdArray<TExtendedMarker, TValue, N>
    where
        TMarker: Extends<TExtendedMarker>,
    {
        IdArray::from_array(self.into_array())
    }
}
