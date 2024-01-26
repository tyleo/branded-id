use crate::{IdPtr, IsizeId};
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

impl<TMarker, TValue> MutIdPtr<TMarker, TValue> {
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

    /// # Safety
    /// This function should ensure that the offset results in a slice which
    /// shares memory with the original slice.
    pub const unsafe fn offset(self, offset: IsizeId<TMarker>) -> Self {
        Self::from_mut_ptr(self.to_mut_ptr().offset(offset.to_isize()))
    }

    pub const fn to_id_ptr(self) -> IdPtr<TMarker, TValue> {
        IdPtr::from_ptr(self.to_mut_ptr())
    }

    pub const fn to_mut_ptr(self) -> *mut TValue {
        self.repr
    }
}

impl<TMarker, TValue> Clone for MutIdPtr<TMarker, TValue> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<TMarker, TValue> Copy for MutIdPtr<TMarker, TValue> {}

impl<TMarker, TValue> Debug for MutIdPtr<TMarker, TValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.to_id_ptr(), f)
    }
}

impl<TMarker, TValue> Eq for MutIdPtr<TMarker, TValue> {}

impl<TMarker, TValue> From<*mut TValue> for MutIdPtr<TMarker, TValue> {
    fn from(value: *mut TValue) -> Self {
        Self::from_mut_ptr(value)
    }
}

impl<TMarker, TValue> Hash for MutIdPtr<TMarker, TValue> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_mut_ptr().hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        let data = unsafe { transmute(data) };
        <*mut TValue>::hash_slice(data, state)
    }
}

impl<TMarker, TValue> Ord for MutIdPtr<TMarker, TValue> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_mut_ptr().cmp(&other.to_mut_ptr())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_mut_ptr(self.to_mut_ptr().max(other.to_mut_ptr()))
    }

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

impl<TMarker, TValue> PartialEq<MutIdPtr<TMarker, TValue>> for MutIdPtr<TMarker, TValue> {
    fn eq(&self, other: &MutIdPtr<TMarker, TValue>) -> bool {
        self.to_mut_ptr().eq(&other.to_mut_ptr())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &MutIdPtr<TMarker, TValue>) -> bool {
        self.to_mut_ptr().ne(&other.to_mut_ptr())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TMarker, TValue> PartialOrd for MutIdPtr<TMarker, TValue> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_mut_ptr().partial_cmp(&other.to_mut_ptr())
    }

    fn lt(&self, other: &Self) -> bool {
        self.to_mut_ptr().lt(&other.to_mut_ptr())
    }

    fn le(&self, other: &Self) -> bool {
        self.to_mut_ptr().le(&other.to_mut_ptr())
    }

    fn gt(&self, other: &Self) -> bool {
        self.to_mut_ptr().gt(&other.to_mut_ptr())
    }

    fn ge(&self, other: &Self) -> bool {
        self.to_mut_ptr().ge(&other.to_mut_ptr())
    }
}

impl<TMarker, TValue> Pointer for MutIdPtr<TMarker, TValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Pointer::fmt(&self.to_id_ptr(), f)
    }
}
