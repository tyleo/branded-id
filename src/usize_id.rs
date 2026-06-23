use crate::{I32Id, IdSlice, IdSliceIndex, IsizeId, U32Id, internal};
use std::{
    mem::transmute,
    ops::{Bound, Index, IndexMut, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive},
};

internal::scalar_id! { UsizeId, usize, from_usize, to_usize }

impl<TMarker: ?Sized> UsizeId<TMarker> {
    pub const fn offset(self, value: usize) -> UsizeId<TMarker> {
        UsizeId::from_usize(self.to_usize() + value)
    }

    pub const fn to_i32_id(self) -> I32Id<TMarker> {
        I32Id::from_i32(self.to_usize() as i32)
    }

    pub const fn to_isize_id(self) -> IsizeId<TMarker> {
        IsizeId::from_isize(self.to_usize() as isize)
    }

    pub const fn to_u32_id(self) -> U32Id<TMarker> {
        U32Id::from_u32(self.to_usize() as u32)
    }

    pub(crate) const fn usize_bound_pair(
        bound_pair: (Bound<UsizeId<TMarker>>, Bound<UsizeId<TMarker>>),
    ) -> (Bound<usize>, Bound<usize>) {
        unsafe { transmute(bound_pair) }
    }

    pub(crate) const fn usize_range(range: Range<UsizeId<TMarker>>) -> Range<usize> {
        unsafe { transmute(range) }
    }

    pub(crate) const fn usize_range_from(range: RangeFrom<UsizeId<TMarker>>) -> RangeFrom<usize> {
        unsafe { transmute(range) }
    }

    pub(crate) const fn usize_range_inclusive(
        range: RangeInclusive<UsizeId<TMarker>>,
    ) -> RangeInclusive<usize> {
        unsafe { transmute(range) }
    }

    pub(crate) const fn usize_range_to(range: RangeTo<UsizeId<TMarker>>) -> RangeTo<usize> {
        unsafe { transmute(range) }
    }

    pub(crate) const fn usize_range_to_inclusive(
        range: RangeToInclusive<UsizeId<TMarker>>,
    ) -> RangeToInclusive<usize> {
        unsafe { transmute(range) }
    }
}

impl<TValue, TMarker: ?Sized> IdSliceIndex<IdSlice<TMarker, TValue>> for UsizeId<TMarker> {
    type Output = TValue;

    fn get(self, slice: &IdSlice<TMarker, TValue>) -> Option<&Self::Output> {
        slice.as_slice().get(self.to_usize())
    }

    fn get_mut(self, slice: &mut IdSlice<TMarker, TValue>) -> Option<&mut Self::Output> {
        slice.as_mut_slice().get_mut(self.to_usize())
    }

    fn index(self, slice: &IdSlice<TMarker, TValue>) -> &Self::Output {
        slice.as_slice().index(self.to_usize())
    }

    fn index_mut(self, slice: &mut IdSlice<TMarker, TValue>) -> &mut Self::Output {
        slice.as_mut_slice().index_mut(self.to_usize())
    }
}
