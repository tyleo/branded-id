use crate::{IdSlice, IdSliceIndex, internal};
use std::{
    mem::transmute,
    ops::{Bound, Index, IndexMut, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive},
};

internal::scalar_id! { UsizeId, usize, from_usize, to_usize }

impl<TBrand: ?Sized> UsizeId<TBrand> {
    /// Returns the id advanced by `value`, keeping the brand.
    ///
    /// # Panics
    /// Panics if the addition overflows `usize`.
    pub const fn offset(self, value: usize) -> UsizeId<TBrand> {
        UsizeId::from_usize(self.to_usize() + value)
    }

    pub(crate) const fn usize_bound_pair(
        bound_pair: (Bound<UsizeId<TBrand>>, Bound<UsizeId<TBrand>>),
    ) -> (Bound<usize>, Bound<usize>) {
        // SAFETY: UsizeId<TBrand> is #[repr(transparent)] over usize, so a
        // Bound parameterized by UsizeId has the same layout as one over
        // usize, and the same holds for the range reinterpretations below.
        unsafe { transmute(bound_pair) }
    }

    pub(crate) const fn usize_range(range: Range<UsizeId<TBrand>>) -> Range<usize> {
        // SAFETY: see usize_bound_pair; UsizeId is #[repr(transparent)] over usize.
        unsafe { transmute(range) }
    }

    pub(crate) const fn usize_range_from(range: RangeFrom<UsizeId<TBrand>>) -> RangeFrom<usize> {
        // SAFETY: see usize_bound_pair; UsizeId is #[repr(transparent)] over usize.
        unsafe { transmute(range) }
    }

    pub(crate) const fn usize_range_inclusive(
        range: RangeInclusive<UsizeId<TBrand>>,
    ) -> RangeInclusive<usize> {
        // SAFETY: see usize_bound_pair; UsizeId is #[repr(transparent)] over usize.
        unsafe { transmute(range) }
    }

    pub(crate) const fn usize_range_to(range: RangeTo<UsizeId<TBrand>>) -> RangeTo<usize> {
        // SAFETY: see usize_bound_pair; UsizeId is #[repr(transparent)] over usize.
        unsafe { transmute(range) }
    }

    pub(crate) const fn usize_range_to_inclusive(
        range: RangeToInclusive<UsizeId<TBrand>>,
    ) -> RangeToInclusive<usize> {
        // SAFETY: see usize_bound_pair; UsizeId is #[repr(transparent)] over usize.
        unsafe { transmute(range) }
    }
}

impl<TValue, TBrand: ?Sized> IdSliceIndex<IdSlice<TBrand, TValue>> for UsizeId<TBrand> {
    type Output = TValue;

    fn get(self, slice: &IdSlice<TBrand, TValue>) -> Option<&Self::Output> {
        slice.as_slice().get(self.to_usize())
    }

    fn get_mut(self, slice: &mut IdSlice<TBrand, TValue>) -> Option<&mut Self::Output> {
        slice.as_mut_slice().get_mut(self.to_usize())
    }

    fn index(self, slice: &IdSlice<TBrand, TValue>) -> &Self::Output {
        slice.as_slice().index(self.to_usize())
    }

    fn index_mut(self, slice: &mut IdSlice<TBrand, TValue>) -> &mut Self::Output {
        slice.as_mut_slice().index_mut(self.to_usize())
    }
}
