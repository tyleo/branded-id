use crate::{IdSlice, IdSliceIndex};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
    ops::{Bound, Index, IndexMut, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive},
};

#[derive(Debug)]
pub struct UsizeId<TMarker: ?Sized> {
    usize: usize,
    phantom: PhantomData<TMarker>,
}

impl<TMarker> UsizeId<TMarker> {
    pub const fn from_usize(usize: usize) -> UsizeId<TMarker> {
        UsizeId {
            usize,
            phantom: PhantomData,
        }
    }

    pub const fn offset(self, value: usize) -> UsizeId<TMarker> {
        UsizeId::from_usize(self.to_usize() + value)
    }

    pub const fn to_usize(self) -> usize {
        self.usize
    }

    pub const fn usize_bound_pair_from_usize_id_bound_pair(
        usize_id_bound_pair: (Bound<UsizeId<TMarker>>, Bound<UsizeId<TMarker>>),
    ) -> (Bound<usize>, Bound<usize>) {
        unsafe { transmute(usize_id_bound_pair) }
    }

    pub const fn usize_range_from_usize_id_range(
        usize_id_range: Range<UsizeId<TMarker>>,
    ) -> Range<usize> {
        unsafe { transmute(usize_id_range) }
    }

    pub const fn usize_range_from_from_usize_id_range_from(
        usize_id_range_from: RangeFrom<UsizeId<TMarker>>,
    ) -> RangeFrom<usize> {
        unsafe { transmute(usize_id_range_from) }
    }

    pub const fn usize_range_inclusive_from_usize_id_range_inclusive(
        usize_id_range_inclusive: RangeInclusive<UsizeId<TMarker>>,
    ) -> RangeInclusive<usize> {
        unsafe { transmute(usize_id_range_inclusive) }
    }

    pub const fn usize_range_to_from_usize_id_range_to(
        usize_id_range_to: RangeTo<UsizeId<TMarker>>,
    ) -> RangeTo<usize> {
        unsafe { transmute(usize_id_range_to) }
    }

    pub const fn usize_range_to_inclusive_from_usize_id_range_to_inclusive(
        usize_id_range_to_inclusive: RangeToInclusive<UsizeId<TMarker>>,
    ) -> RangeToInclusive<usize> {
        unsafe { transmute(usize_id_range_to_inclusive) }
    }
}

impl<TMarker> Clone for UsizeId<TMarker> {
    fn clone(&self) -> UsizeId<TMarker> {
        *self
    }
}

impl<TMarker> Copy for UsizeId<TMarker> {}

impl<TMarker> Eq for UsizeId<TMarker> {}

impl<TMarker> From<usize> for UsizeId<TMarker> {
    fn from(val: usize) -> Self {
        UsizeId::from_usize(val)
    }
}

impl<TMarker> Hash for UsizeId<TMarker> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.usize.hash(state)
    }
}

unsafe impl<TValue, TMarker> IdSliceIndex<IdSlice<TMarker, TValue>> for UsizeId<TMarker> {
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

impl<TMarker> Ord for UsizeId<TMarker> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.usize.cmp(&other.usize)
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.usize.max(other.usize).into()
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.usize.min(other.usize).into()
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        self.usize.clamp(min.usize, max.usize).into()
    }
}

impl<TMarker> PartialEq for UsizeId<TMarker> {
    fn eq(&self, other: &Self) -> bool {
        self.usize == other.usize
    }
}

impl<TMarker> PartialOrd for UsizeId<TMarker> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        self.usize.lt(&other.usize)
    }

    fn le(&self, other: &Self) -> bool {
        self.usize.le(&other.usize)
    }

    fn gt(&self, other: &Self) -> bool {
        self.usize.gt(&other.usize)
    }

    fn ge(&self, other: &Self) -> bool {
        self.usize.ge(&other.usize)
    }
}
