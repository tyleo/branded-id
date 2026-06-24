use crate::{
    I8Id, I16Id, I32Id, I64Id, I128Id, IsizeId, U8Id, U16Id, U32Id, U64Id, U128Id, UsizeId,
};
use std::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

// Implemented only for the types this crate's sealed traits target, so that
// outside crates cannot implement those traits. This lets us add more trait
// methods in the future without it being a breaking change.
pub trait Sealed {}

impl Sealed for i8 {}
impl Sealed for i16 {}
impl Sealed for i32 {}
impl Sealed for i64 {}
impl Sealed for i128 {}
impl Sealed for isize {}
impl Sealed for u8 {}
impl Sealed for u16 {}
impl Sealed for u32 {}
impl Sealed for u64 {}
impl Sealed for u128 {}
impl Sealed for usize {}

impl<TValue> Sealed for [TValue] {}
impl<TValue, const N: usize> Sealed for [TValue; N] {}
impl<TValue> Sealed for Vec<TValue> {}

impl<TValue> Sealed for *const TValue {}
impl<TValue> Sealed for *mut TValue {}

impl<TBrand: ?Sized> Sealed for I8Id<TBrand> {}
impl<TBrand: ?Sized> Sealed for I16Id<TBrand> {}
impl<TBrand: ?Sized> Sealed for I32Id<TBrand> {}
impl<TBrand: ?Sized> Sealed for I64Id<TBrand> {}
impl<TBrand: ?Sized> Sealed for I128Id<TBrand> {}
impl<TBrand: ?Sized> Sealed for IsizeId<TBrand> {}
impl<TBrand: ?Sized> Sealed for U8Id<TBrand> {}
impl<TBrand: ?Sized> Sealed for U16Id<TBrand> {}
impl<TBrand: ?Sized> Sealed for U32Id<TBrand> {}
impl<TBrand: ?Sized> Sealed for U64Id<TBrand> {}
impl<TBrand: ?Sized> Sealed for U128Id<TBrand> {}
impl<TBrand: ?Sized> Sealed for UsizeId<TBrand> {}

impl<TBrand: ?Sized> Sealed for Range<UsizeId<TBrand>> {}
impl<TBrand: ?Sized> Sealed for RangeFrom<UsizeId<TBrand>> {}
impl<TBrand: ?Sized> Sealed for RangeInclusive<UsizeId<TBrand>> {}
impl<TBrand: ?Sized> Sealed for RangeTo<UsizeId<TBrand>> {}
impl<TBrand: ?Sized> Sealed for RangeToInclusive<UsizeId<TBrand>> {}
impl Sealed for RangeFull {}
impl<TBrand: ?Sized> Sealed for (Bound<UsizeId<TBrand>>, Bound<UsizeId<TBrand>>) {}
