use crate::IdPtr;
use std::{
    cmp::Ordering,
    fmt::{self, Debug, Pointer},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
};

/// A `*mut TValue` raw pointer tagged with an id brand.
#[repr(transparent)]
pub struct MutIdPtr<TBrand: ?Sized, TValue: ?Sized> {
    phantom: PhantomData<TBrand>,
    repr: *mut TValue,
}

impl<TBrand: ?Sized, TValue: ?Sized> MutIdPtr<TBrand, TValue> {
    pub const fn cast_to<TValue2>(self) -> MutIdPtr<TBrand, TValue2> {
        MutIdPtr::from_mut_ptr(self.to_mut_ptr() as *mut TValue2)
    }

    /// # Safety
    /// The same rules apply as dereferencing a raw pointer.
    pub const unsafe fn deref_ptr<'a>(self) -> &'a TValue {
        unsafe { &*self.repr }
    }

    /// # Safety
    /// The same rules apply as dereferencing a raw pointer.
    pub const unsafe fn deref_ptr_mut<'a>(self) -> &'a mut TValue {
        unsafe { &mut *self.repr }
    }

    pub const fn from_mut_ptr(repr: *mut TValue) -> Self {
        Self {
            phantom: PhantomData,
            repr,
        }
    }

    pub const fn to_id_ptr(self) -> IdPtr<TBrand, TValue> {
        IdPtr::from_ptr(self.to_mut_ptr())
    }

    pub const fn to_mut_ptr(self) -> *mut TValue {
        self.repr
    }
}

impl<TBrand: ?Sized, TValue> MutIdPtr<TBrand, TValue> {
    /// # Safety
    /// See <https://doc.rust-lang.org/std/primitive.pointer.html#method.read-1>
    pub const unsafe fn read(self) -> TValue {
        unsafe { self.to_mut_ptr().read() }
    }

    /// # Safety
    /// See <https://doc.rust-lang.org/std/primitive.pointer.html#method.read_unaligned-1>
    pub const unsafe fn read_unaligned(self) -> TValue {
        unsafe { self.to_mut_ptr().read_unaligned() }
    }

    /// # Safety
    /// See <https://doc.rust-lang.org/std/primitive.pointer.html#method.write>
    pub unsafe fn write(self, val: TValue) {
        unsafe { self.to_mut_ptr().write(val) }
    }

    /// # Safety
    /// See <https://doc.rust-lang.org/std/primitive.pointer.html#method.write_unaligned>
    pub unsafe fn write_unaligned(self, val: TValue) {
        unsafe { self.to_mut_ptr().write_unaligned(val) }
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> Clone for MutIdPtr<TBrand, TValue> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> Copy for MutIdPtr<TBrand, TValue> {}

impl<TBrand: ?Sized, TValue: ?Sized> Debug for MutIdPtr<TBrand, TValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.to_id_ptr(), f)
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> Eq for MutIdPtr<TBrand, TValue> {}

impl<TBrand: ?Sized, TValue: ?Sized> From<*mut TValue> for MutIdPtr<TBrand, TValue> {
    fn from(value: *mut TValue) -> Self {
        Self::from_mut_ptr(value)
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> Hash for MutIdPtr<TBrand, TValue> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_mut_ptr().hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        let data = unsafe { transmute::<&[MutIdPtr<TBrand, TValue>], &[*mut TValue]>(data) };
        <*mut TValue>::hash_slice(data, state)
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> Ord for MutIdPtr<TBrand, TValue> {
    #[allow(ambiguous_wide_pointer_comparisons)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_mut_ptr().cmp(&other.to_mut_ptr())
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_mut_ptr(self.to_mut_ptr().max(other.to_mut_ptr()))
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_mut_ptr(self.to_mut_ptr().min(other.to_mut_ptr()))
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        Self::from_mut_ptr(self.to_mut_ptr().clamp(min.to_mut_ptr(), max.to_mut_ptr()))
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> PartialEq<MutIdPtr<TBrand, TValue>>
    for MutIdPtr<TBrand, TValue>
{
    #[allow(ambiguous_wide_pointer_comparisons)]
    fn eq(&self, other: &MutIdPtr<TBrand, TValue>) -> bool {
        self.to_mut_ptr().eq(&other.to_mut_ptr())
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &MutIdPtr<TBrand, TValue>) -> bool {
        self.to_mut_ptr().ne(&other.to_mut_ptr())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TBrand: ?Sized, TValue: ?Sized> PartialOrd for MutIdPtr<TBrand, TValue> {
    #[allow(ambiguous_wide_pointer_comparisons)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_mut_ptr().partial_cmp(&other.to_mut_ptr())
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    fn lt(&self, other: &Self) -> bool {
        self.to_mut_ptr().lt(&other.to_mut_ptr())
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    fn le(&self, other: &Self) -> bool {
        self.to_mut_ptr().le(&other.to_mut_ptr())
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    fn gt(&self, other: &Self) -> bool {
        self.to_mut_ptr().gt(&other.to_mut_ptr())
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    fn ge(&self, other: &Self) -> bool {
        self.to_mut_ptr().ge(&other.to_mut_ptr())
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> Pointer for MutIdPtr<TBrand, TValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Pointer::fmt(&self.to_id_ptr(), f)
    }
}
