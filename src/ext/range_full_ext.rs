use crate::{IdSlice, IdSliceIndex};
use std::ops::RangeFull;

impl<TMarker: ?Sized, TValue> IdSliceIndex<IdSlice<TMarker, TValue>> for RangeFull {
    type Output = IdSlice<TMarker, TValue>;

    fn get(self, slice: &IdSlice<TMarker, TValue>) -> Option<&Self::Output> {
        let slice = slice.as_slice().get(self)?;
        let id_slice = IdSlice::from_slice(slice);
        Some(id_slice)
    }

    fn get_mut(self, slice: &mut IdSlice<TMarker, TValue>) -> Option<&mut Self::Output> {
        let mut_slice = slice.as_mut_slice().get_mut(self)?;
        let mut_id_slice = IdSlice::from_mut_slice(mut_slice);
        Some(mut_id_slice)
    }

    fn index(self, slice: &IdSlice<TMarker, TValue>) -> &Self::Output {
        IdSlice::from_slice(slice.as_slice())
    }

    fn index_mut(self, slice: &mut IdSlice<TMarker, TValue>) -> &mut Self::Output {
        IdSlice::from_mut_slice(slice.as_mut_slice())
    }
}
