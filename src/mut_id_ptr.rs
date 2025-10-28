use crate::{IdPtr, IsizeId, UsizeId};
use std::{
    cmp::Ordering,
    fmt::{self, Debug, Pointer},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
};

pub struct MutIdPtr<TMarker: ?Sized, TValue: ?Sized> {
    phantom: PhantomData<TMarker>,
    repr: *mut TValue,
}

impl<TMarker: ?Sized, TValue: ?Sized> MutIdPtr<TMarker, TValue> {
    pub const fn cast_to<TValue2>(self) -> MutIdPtr<TMarker, TValue2> {
        MutIdPtr::from_mut_ptr(self.to_mut_ptr() as *mut TValue2)
    }

    /// # Safety
    /// The same rules apply as dereferencing a raw pointer.
    pub unsafe fn deref_ptr<'a>(self) -> &'a mut TValue {
        &mut *self.repr
    }

    /// # Safety
    /// The same rules apply as dereferencing a raw pointer.
    pub unsafe fn deref_ptr_mut<'a>(self) -> &'a mut TValue {
        &mut *self.repr
    }

    pub const fn from_mut_ptr(repr: *mut TValue) -> Self {
        Self {
            phantom: PhantomData,
            repr,
        }
    }

    pub const fn to_id_ptr(self) -> IdPtr<TMarker, TValue> {
        IdPtr::from_ptr(self.to_mut_ptr())
    }

    pub const fn to_mut_ptr(self) -> *mut TValue {
        self.repr
    }
}

impl<TMarker: ?Sized, TValue> MutIdPtr<TMarker, TValue> {
    /// # Safety
    /// See https://doc.rust-lang.org/std/primitive.pointer.html#method.add-1
    pub const unsafe fn add(self, count: UsizeId<TMarker>) -> MutIdPtr<TMarker, TValue> {
        MutIdPtr::from_mut_ptr(self.to_mut_ptr().add(count.to_usize()))
    }

    /// # Safety
    /// See https://doc.rust-lang.org/std/primitive.pointer.html#method.offset-1
    pub const unsafe fn offset(self, offset: IsizeId<TMarker>) -> Self {
        Self::from_mut_ptr(self.to_mut_ptr().offset(offset.to_isize()))
    }

    /// # Safety
    /// See https://doc.rust-lang.org/std/primitive.pointer.html#method.read-1
    pub const unsafe fn read(self) -> TValue {
        self.to_mut_ptr().read()
    }

    /// # Safety
    /// See https://doc.rust-lang.org/std/primitive.pointer.html#method.read_unaligned-1
    pub const unsafe fn read_unaligned(self) -> TValue {
        self.to_mut_ptr().read_unaligned()
    }

    /// # Safety
    /// See https://doc.rust-lang.org/std/primitive.pointer.html#method.write
    pub unsafe fn write(self, val: TValue) {
        self.to_mut_ptr().write(val)
    }

    /// # Safety
    /// See https://doc.rust-lang.org/std/primitive.pointer.html#method.write_unaligned
    pub unsafe fn write_unaligned(self, val: TValue) {
        self.to_mut_ptr().write_unaligned(val)
    }
}

impl<TMarker: ?Sized, TValue: ?Sized> Clone for MutIdPtr<TMarker, TValue> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TMarker: ?Sized, TValue: ?Sized> Copy for MutIdPtr<TMarker, TValue> {}

impl<TMarker: ?Sized, TValue: ?Sized> Debug for MutIdPtr<TMarker, TValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.to_id_ptr(), f)
    }
}

impl<TMarker: ?Sized, TValue: ?Sized> Eq for MutIdPtr<TMarker, TValue> {}

impl<TMarker: ?Sized, TValue: ?Sized> From<*mut TValue> for MutIdPtr<TMarker, TValue> {
    fn from(value: *mut TValue) -> Self {
        Self::from_mut_ptr(value)
    }
}

impl<TMarker: ?Sized, TValue: ?Sized> Hash for MutIdPtr<TMarker, TValue> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_mut_ptr().hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        let data = unsafe { transmute::<&[MutIdPtr<TMarker, TValue>], &[*mut TValue]>(data) };
        <*mut TValue>::hash_slice(data, state)
    }
}

impl<TMarker: ?Sized, TValue: ?Sized> Ord for MutIdPtr<TMarker, TValue> {
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

impl<TMarker: ?Sized, TValue: ?Sized> PartialEq<MutIdPtr<TMarker, TValue>>
    for MutIdPtr<TMarker, TValue>
{
    #[allow(ambiguous_wide_pointer_comparisons)]
    fn eq(&self, other: &MutIdPtr<TMarker, TValue>) -> bool {
        self.to_mut_ptr().eq(&other.to_mut_ptr())
    }

    #[allow(ambiguous_wide_pointer_comparisons)]
    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &MutIdPtr<TMarker, TValue>) -> bool {
        self.to_mut_ptr().ne(&other.to_mut_ptr())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TMarker: ?Sized, TValue: ?Sized> PartialOrd for MutIdPtr<TMarker, TValue> {
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

impl<TMarker: ?Sized, TValue: ?Sized> Pointer for MutIdPtr<TMarker, TValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Pointer::fmt(&self.to_id_ptr(), f)
    }
}
