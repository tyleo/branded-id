use crate::UsizeId;
use std::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

// Implemented only for the types this crate's extension traits target, so that
// outside crates cannot implement those sealed traits. This lets us add more
// trait methods in the future without it being a breaking change.
pub trait Sealed {}

impl Sealed for i32 {}
impl Sealed for isize {}
impl Sealed for u32 {}
impl Sealed for usize {}

impl<TValue> Sealed for [TValue] {}
impl<TValue, const N: usize> Sealed for [TValue; N] {}
impl<TValue> Sealed for Vec<TValue> {}

impl<TValue> Sealed for *const TValue {}
impl<TValue> Sealed for *mut TValue {}

impl<TMarker: ?Sized> Sealed for UsizeId<TMarker> {}

impl<TMarker: ?Sized> Sealed for Range<UsizeId<TMarker>> {}
impl<TMarker: ?Sized> Sealed for RangeFrom<UsizeId<TMarker>> {}
impl<TMarker: ?Sized> Sealed for RangeInclusive<UsizeId<TMarker>> {}
impl<TMarker: ?Sized> Sealed for RangeTo<UsizeId<TMarker>> {}
impl<TMarker: ?Sized> Sealed for RangeToInclusive<UsizeId<TMarker>> {}
impl Sealed for RangeFull {}
impl<TMarker: ?Sized> Sealed for (Bound<UsizeId<TMarker>>, Bound<UsizeId<TMarker>>) {}
