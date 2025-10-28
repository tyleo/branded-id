use crate::{internal::fmt_marker_name, IsizeId, MutIdPtr, UsizeId};
use std::{
    cmp::Ordering,
    fmt::{self, Debug, Pointer, Write},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
};

pub struct IdPtr<TMarker: ?Sized, TValue: ?Sized> {
    phantom: PhantomData<TMarker>,
    repr: *const TValue,
}

impl<TMarker: ?Sized, TValue> IdPtr<TMarker, TValue> {
    /// # Safety
    /// See https://doc.rust-lang.org/std/primitive.pointer.html#method.add
    pub const unsafe fn add(self, count: UsizeId<TMarker>) -> IdPtr<TMarker, TValue> {
        IdPtr::from_ptr(self.to_ptr().add(count.to_usize()))
    }

    pub const fn cast_to<TValue2>(self) -> IdPtr<TMarker, TValue2> {
        IdPtr::from_ptr(self.to_ptr() as *const TValue2)
    }

    /// # Safety
    /// The same rules apply as dereferencing a raw pointer.
    pub const unsafe fn deref_ptr<'a>(self) -> &'a TValue {
        &*self.repr
    }

    pub const fn from_ptr(repr: *const TValue) -> Self {
        Self {
            phantom: PhantomData,
            repr,
        }
    }

    /// # Safety
    /// See https://doc.rust-lang.org/std/primitive.pointer.html#method.offset
    pub const unsafe fn offset(self, offset: IsizeId<TMarker>) -> Self {
        Self::from_ptr(self.to_ptr().offset(offset.to_isize()))
    }

    /// # Safety
    /// See https://doc.rust-lang.org/std/primitive.pointer.html#method.read
    pub const unsafe fn read(self) -> TValue {
        self.to_ptr().read()
    }

    /// # Safety
    /// See https://doc.rust-lang.org/std/primitive.pointer.html#method.read_unaligned
    pub const unsafe fn read_unaligned(self) -> TValue {
        self.to_ptr().read_unaligned()
    }

    pub const fn to_ptr(self) -> *const TValue {
        self.repr
    }
}

impl<TMarker: ?Sized, TValue> Clone for IdPtr<TMarker, TValue> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TMarker: ?Sized, TValue> Copy for IdPtr<TMarker, TValue> {}

impl<TMarker: ?Sized, TValue> Debug for IdPtr<TMarker, TValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_marker_name::<TMarker>(f)?;
        f.write_char('(')?;
        Debug::fmt(&self.to_ptr(), f)?;
        f.write_char(')')
    }
}

impl<TMarker: ?Sized, TValue> Eq for IdPtr<TMarker, TValue> {}

impl<TMarker: ?Sized, TValue> From<*const TValue> for IdPtr<TMarker, TValue> {
    fn from(value: *const TValue) -> Self {
        Self::from_ptr(value)
    }
}

impl<TMarker: ?Sized, TValue> From<*mut TValue> for IdPtr<TMarker, TValue> {
    fn from(value: *mut TValue) -> Self {
        Self::from_ptr(value)
    }
}

impl<TMarker: ?Sized, TValue> From<MutIdPtr<TMarker, TValue>> for IdPtr<TMarker, TValue> {
    fn from(value: MutIdPtr<TMarker, TValue>) -> Self {
        Self::from_ptr(value.to_mut_ptr())
    }
}

impl<TMarker: ?Sized, TValue> Hash for IdPtr<TMarker, TValue> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_ptr().hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        let data = unsafe { transmute::<&[IdPtr<TMarker, TValue>], &[*const TValue]>(data) };
        <*const TValue>::hash_slice(data, state)
    }
}

impl<TMarker: ?Sized, TValue> Ord for IdPtr<TMarker, TValue> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_ptr().cmp(&other.to_ptr())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_ptr(self.to_ptr().max(other.to_ptr()))
    }

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

impl<TMarker: ?Sized, TValue> PartialEq<IdPtr<TMarker, TValue>> for IdPtr<TMarker, TValue> {
    fn eq(&self, other: &IdPtr<TMarker, TValue>) -> bool {
        self.to_ptr().eq(&other.to_ptr())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdPtr<TMarker, TValue>) -> bool {
        self.to_ptr().ne(&other.to_ptr())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TMarker: ?Sized, TValue> PartialOrd for IdPtr<TMarker, TValue> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_ptr().partial_cmp(&other.to_ptr())
    }

    fn lt(&self, other: &Self) -> bool {
        self.to_ptr().lt(&other.to_ptr())
    }

    fn le(&self, other: &Self) -> bool {
        self.to_ptr().le(&other.to_ptr())
    }

    fn gt(&self, other: &Self) -> bool {
        self.to_ptr().gt(&other.to_ptr())
    }

    fn ge(&self, other: &Self) -> bool {
        self.to_ptr().ge(&other.to_ptr())
    }
}

impl<TMarker: ?Sized, TValue> Pointer for IdPtr<TMarker, TValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_marker_name::<TMarker>(f)?;
        f.write_char('(')?;
        Debug::fmt(&self.to_ptr(), f)?;
        f.write_char(')')
    }
}
