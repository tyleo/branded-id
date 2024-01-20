use crate::internal::Sealed;

pub unsafe trait IdSliceIndex<T>: Sealed
where
    T: ?Sized,
{
    type Output: ?Sized;

    fn get(self, slice: &T) -> Option<&Self::Output>;

    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;

    fn index(self, slice: &T) -> &Self::Output;

    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}
