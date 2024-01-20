use crate::{IdSlice, IdSliceIndex, UsizeId};
use std::ops::RangeToInclusive;

unsafe impl<TMarker, TValue> IdSliceIndex<IdSlice<TMarker, TValue>>
    for RangeToInclusive<UsizeId<TMarker>>
{
    type Output = IdSlice<TMarker, TValue>;

    fn get(self, slice: &IdSlice<TMarker, TValue>) -> Option<&Self::Output> {
        let usize_range = UsizeId::usize_range_to_inclusive_from_usize_id_range_to_inclusive(self);
        let slice = slice.as_slice().get(usize_range)?;
        let id_slice = IdSlice::from_slice(slice);
        Some(id_slice)
    }

    fn get_mut(self, slice: &mut IdSlice<TMarker, TValue>) -> Option<&mut Self::Output> {
        let usize_range = UsizeId::usize_range_to_inclusive_from_usize_id_range_to_inclusive(self);
        let mut_slice = slice.as_mut_slice().get_mut(usize_range)?;
        let mut_id_slice = IdSlice::from_mut_slice(mut_slice);
        Some(mut_id_slice)
    }

    fn index(self, slice: &IdSlice<TMarker, TValue>) -> &Self::Output {
        let usize_range = UsizeId::usize_range_to_inclusive_from_usize_id_range_to_inclusive(self);
        IdSlice::from_slice(&slice.as_slice()[usize_range])
    }

    fn index_mut(self, slice: &mut IdSlice<TMarker, TValue>) -> &mut Self::Output {
        let usize_range = UsizeId::usize_range_to_inclusive_from_usize_id_range_to_inclusive(self);
        IdSlice::from_mut_slice(&mut slice.as_mut_slice()[usize_range])
    }
}
