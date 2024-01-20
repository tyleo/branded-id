use crate::{IdSlice, IdSliceIndex, UsizeId};
use std::ops::Bound;

unsafe impl<TMarker, TValue> IdSliceIndex<IdSlice<TMarker, TValue>>
    for (Bound<UsizeId<TMarker>>, Bound<UsizeId<TMarker>>)
{
    type Output = IdSlice<TMarker, TValue>;

    fn get(self, slice: &IdSlice<TMarker, TValue>) -> Option<&Self::Output> {
        let usize_bound_pair = UsizeId::usize_bound_pair_from_usize_id_bound_pair(self);
        let slice = slice.as_slice().get(usize_bound_pair)?;
        let id_slice = IdSlice::from_slice(slice);
        Some(id_slice)
    }

    fn get_mut(self, slice: &mut IdSlice<TMarker, TValue>) -> Option<&mut Self::Output> {
        let usize_bound_pair = UsizeId::usize_bound_pair_from_usize_id_bound_pair(self);
        let mut_slice = slice.as_mut_slice().get_mut(usize_bound_pair)?;
        let mut_id_slice = IdSlice::from_mut_slice(mut_slice);
        Some(mut_id_slice)
    }

    fn index(self, slice: &IdSlice<TMarker, TValue>) -> &Self::Output {
        let usize_bound_pair = UsizeId::usize_bound_pair_from_usize_id_bound_pair(self);
        IdSlice::from_slice(&slice.as_slice()[usize_bound_pair])
    }

    fn index_mut(self, slice: &mut IdSlice<TMarker, TValue>) -> &mut Self::Output {
        let usize_bound_pair = UsizeId::usize_bound_pair_from_usize_id_bound_pair(self);
        IdSlice::from_mut_slice(&mut slice.as_mut_slice()[usize_bound_pair])
    }
}
