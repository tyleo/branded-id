use crate::{I32Id, IdSlice, IdSliceIndex, IsizeId, U32Id, internal::fmt_marker_name};
use std::{
    cmp::Ordering,
    fmt::{
        self, Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex,
        Write,
    },
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
    ops::{Bound, Index, IndexMut, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive},
    str::FromStr,
};

#[repr(transparent)]
pub struct UsizeId<TMarker: ?Sized> {
    phantom: PhantomData<TMarker>,
    repr: usize,
}

impl<TMarker: ?Sized> UsizeId<TMarker> {
    fn fmt_helper(
        self,
        fmt_repr: impl FnOnce(&usize, &mut Formatter) -> fmt::Result,
        f: &mut Formatter,
    ) -> fmt::Result {
        fmt_marker_name::<TMarker>(f)?;
        f.write_char('(')?;
        fmt_repr(&self.to_usize(), f)?;
        f.write_char(')')
    }

    pub const fn from_usize(usize: usize) -> UsizeId<TMarker> {
        Self {
            phantom: PhantomData,
            repr: usize,
        }
    }

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

    pub const fn to_usize(self) -> usize {
        self.repr
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

impl<TMarker: ?Sized> Binary for UsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Binary::fmt, f)
    }
}

impl<TMarker: ?Sized> Clone for UsizeId<TMarker> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TMarker: ?Sized> Copy for UsizeId<TMarker> {}

impl<TMarker: ?Sized> Debug for UsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Debug::fmt, f)
    }
}

impl<TMarker: ?Sized> Display for UsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Display::fmt, f)
    }
}

impl<TMarker: ?Sized> Eq for UsizeId<TMarker> {}

impl<TMarker: ?Sized> From<usize> for UsizeId<TMarker> {
    fn from(val: usize) -> Self {
        Self::from_usize(val)
    }
}

impl<TMarker: ?Sized> FromStr for UsizeId<TMarker> {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UsizeId::from_usize(usize::from_str(s)?))
    }
}

impl<TMarker: ?Sized> Hash for UsizeId<TMarker> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.to_usize().hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        let data = unsafe { transmute::<&[UsizeId<TMarker>], &[usize]>(data) };
        usize::hash_slice(data, state)
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

impl<TMarker: ?Sized> LowerExp for UsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(LowerExp::fmt, f)
    }
}

impl<TMarker: ?Sized> LowerHex for UsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(LowerHex::fmt, f)
    }
}

impl<TMarker: ?Sized> Octal for UsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(Octal::fmt, f)
    }
}

impl<TMarker: ?Sized> Ord for UsizeId<TMarker> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_usize().cmp(&other.to_usize())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_usize(self.to_usize().max(other.to_usize()))
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_usize(self.to_usize().min(other.to_usize()))
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        Self::from_usize(self.to_usize().clamp(min.to_usize(), max.to_usize()))
    }
}

impl<TMarker: ?Sized> PartialEq for UsizeId<TMarker> {
    fn eq(&self, other: &Self) -> bool {
        self.to_usize().eq(&other.to_usize())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        self.to_usize().ne(&other.to_usize())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TMarker: ?Sized> PartialOrd for UsizeId<TMarker> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_usize().partial_cmp(&other.to_usize())
    }

    fn lt(&self, other: &Self) -> bool {
        self.to_usize().lt(&other.to_usize())
    }

    fn le(&self, other: &Self) -> bool {
        self.to_usize().le(&other.to_usize())
    }

    fn gt(&self, other: &Self) -> bool {
        self.to_usize().gt(&other.to_usize())
    }

    fn ge(&self, other: &Self) -> bool {
        self.to_usize().ge(&other.to_usize())
    }
}

impl<TMarker: ?Sized> UpperExp for UsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(UpperExp::fmt, f)
    }
}

impl<TMarker: ?Sized> UpperHex for UsizeId<TMarker> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_helper(UpperHex::fmt, f)
    }
}
