use crate::{MutIdPtr, internal::fmt_brand_name};
use std::{
    cmp::Ordering,
    fmt::{self, Debug, Pointer, Write},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
};

/// A `*const TValue` raw pointer tagged with an id brand.
#[repr(transparent)]
pub struct IdPtr<TBrand: ?Sized, TValue: ?Sized> {
    phantom: PhantomData<TBrand>,
    repr: *const TValue,
}

impl<TBrand: ?Sized, TValue: ?Sized> IdPtr<TBrand, TValue> {
    /// Reinterprets the pointee as `TValue2`, keeping the brand.
    pub const fn cast_to<TValue2>(self) -> IdPtr<TBrand, TValue2> {
        IdPtr::from_ptr(self.to_ptr() as *const TValue2)
    }

    /// # Safety
    /// The same rules apply as dereferencing a raw pointer.
    pub const unsafe fn deref_ptr<'a>(self) -> &'a TValue {
        unsafe { &*self.repr }
    }

    /// Wraps a raw `*const TValue` with the brand.
    pub const fn from_ptr(repr: *const TValue) -> Self {
        Self {
            phantom: PhantomData,
            repr,
        }
    }

    /// Returns the underlying raw `*const TValue`, dropping the brand.
    pub const fn to_ptr(self) -> *const TValue {
        self.repr
    }
}

impl<TBrand: ?Sized, TValue> IdPtr<TBrand, TValue> {
    /// # Safety
    /// See <https://doc.rust-lang.org/std/primitive.pointer.html#method.read>
    #[must_use]
    pub const unsafe fn read(self) -> TValue {
        unsafe { self.to_ptr().read() }
    }

    /// # Safety
    /// See <https://doc.rust-lang.org/std/primitive.pointer.html#method.read_unaligned>
    #[must_use]
    pub const unsafe fn read_unaligned(self) -> TValue {
        unsafe { self.to_ptr().read_unaligned() }
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> Clone for IdPtr<TBrand, TValue> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> Copy for IdPtr<TBrand, TValue> {}

impl<TBrand: ?Sized, TValue: ?Sized> Debug for IdPtr<TBrand, TValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_brand_name::<TBrand>(f)?;
        f.write_char('(')?;
        Debug::fmt(&self.to_ptr(), f)?;
        f.write_char(')')
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> Eq for IdPtr<TBrand, TValue> {}

impl<TBrand: ?Sized, TValue: ?Sized> From<*const TValue> for IdPtr<TBrand, TValue> {
    fn from(value: *const TValue) -> Self {
        Self::from_ptr(value)
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> From<*mut TValue> for IdPtr<TBrand, TValue> {
    fn from(value: *mut TValue) -> Self {
        Self::from_ptr(value)
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> From<MutIdPtr<TBrand, TValue>> for IdPtr<TBrand, TValue> {
    fn from(value: MutIdPtr<TBrand, TValue>) -> Self {
        Self::from_ptr(value.to_mut_ptr())
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> Hash for IdPtr<TBrand, TValue> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_ptr().hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        // SAFETY: IdPtr is #[repr(transparent)] over *const TValue, so &[IdPtr]
        // and &[*const TValue] share a layout.
        let data = unsafe { transmute::<&[IdPtr<TBrand, TValue>], &[*const TValue]>(data) };
        <*const TValue>::hash_slice(data, state)
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> Ord for IdPtr<TBrand, TValue> {
    #[allow(ambiguous_wide_pointer_comparisons)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_ptr().cmp(&other.to_ptr())
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_ptr(self.to_ptr().max(other.to_ptr()))
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_ptr(self.to_ptr().min(other.to_ptr()))
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        Self::from_ptr(self.to_ptr().clamp(min.to_ptr(), max.to_ptr()))
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> PartialEq<IdPtr<TBrand, TValue>> for IdPtr<TBrand, TValue> {
    #[allow(ambiguous_wide_pointer_comparisons)]
    fn eq(&self, other: &IdPtr<TBrand, TValue>) -> bool {
        self.to_ptr().eq(&other.to_ptr())
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdPtr<TBrand, TValue>) -> bool {
        self.to_ptr().ne(&other.to_ptr())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TBrand: ?Sized, TValue: ?Sized> PartialOrd for IdPtr<TBrand, TValue> {
    #[allow(ambiguous_wide_pointer_comparisons)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_ptr().partial_cmp(&other.to_ptr())
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    fn lt(&self, other: &Self) -> bool {
        self.to_ptr().lt(&other.to_ptr())
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    fn le(&self, other: &Self) -> bool {
        self.to_ptr().le(&other.to_ptr())
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    fn gt(&self, other: &Self) -> bool {
        self.to_ptr().gt(&other.to_ptr())
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    fn ge(&self, other: &Self) -> bool {
        self.to_ptr().ge(&other.to_ptr())
    }
}

impl<TBrand: ?Sized, TValue: ?Sized> Pointer for IdPtr<TBrand, TValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}
