use crate::{IdSlice, IdSliceIndex, UsizeId};
use std::ops::Bound;

impl<TBrand: ?Sized, TValue> IdSliceIndex<IdSlice<TBrand, TValue>>
    for (Bound<UsizeId<TBrand>>, Bound<UsizeId<TBrand>>)
{
    type Output = IdSlice<TBrand, TValue>;

    fn get(self, slice: &IdSlice<TBrand, TValue>) -> Option<&Self::Output> {
        let usize_bound_pair = UsizeId::usize_bound_pair(self);
        let slice = slice.as_slice().get(usize_bound_pair)?;
        let id_slice = IdSlice::from_slice(slice);
        Some(id_slice)
    }

    fn get_mut(self, slice: &mut IdSlice<TBrand, TValue>) -> Option<&mut Self::Output> {
        let usize_bound_pair = UsizeId::usize_bound_pair(self);
        let mut_slice = slice.as_mut_slice().get_mut(usize_bound_pair)?;
        let mut_id_slice = IdSlice::from_mut_slice(mut_slice);
        Some(mut_id_slice)
    }

    fn index(self, slice: &IdSlice<TBrand, TValue>) -> &Self::Output {
        let usize_bound_pair = UsizeId::usize_bound_pair(self);
        IdSlice::from_slice(&slice.as_slice()[usize_bound_pair])
    }

    fn index_mut(self, slice: &mut IdSlice<TBrand, TValue>) -> &mut Self::Output {
        let usize_bound_pair = UsizeId::usize_bound_pair(self);
        IdSlice::from_mut_slice(&mut slice.as_mut_slice()[usize_bound_pair])
    }
}
