use crate::internal::Sealed;

/// A value that can index an [`IdSlice`](crate::IdSlice): a
/// [`UsizeId`](crate::UsizeId) selects one element, an id range selects a
/// sub-slice.
pub trait IdSliceIndex<T>: Sealed
where
    T: ?Sized,
{
    /// The element or sub-slice this index selects.
    type Output: ?Sized;

    /// Returns a shared reference to the selection, or [`None`] if out of bounds.
    fn get(self, slice: &T) -> Option<&Self::Output>;

    /// Returns a mutable reference to the selection, or [`None`] if out of bounds.
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;

    /// Returns a shared reference to the selection.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    fn index(self, slice: &T) -> &Self::Output;

    /// Returns a mutable reference to the selection.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}
