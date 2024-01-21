use crate::IdPtr;
use std::marker::PhantomData;

use super::IsizeId;

#[derive(Debug)]
pub struct MutIdPtr<TMarker: ?Sized, TValue: ?Sized> {
    phantom: PhantomData<TMarker>,
    repr: *mut TValue,
}

impl<TMarker, TValue> MutIdPtr<TMarker, TValue> {
    pub const fn from_mut_ptr(repr: *mut TValue) -> Self {
        Self {
            phantom: PhantomData,
            repr,
        }
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

    /// # Safety
    /// This function should ensure that the offset results in a slice which
    /// shares memory with the original slice.
    pub const unsafe fn offset(self, offset: IsizeId<TMarker>) -> Self {
        let ptr = self.to_mut_ptr().offset(offset.to_isize());
        Self::from_mut_ptr(ptr)
    }

    pub const fn to_id_ptr(self) -> IdPtr<TMarker, TValue> {
        IdPtr::from_ptr(self.to_mut_ptr())
    }

    pub const fn to_mut_ptr(self) -> *mut TValue {
        self.repr
    }
}

impl<TMarker, TValue> Eq for MutIdPtr<TMarker, TValue> where *mut TValue: PartialEq {}

impl<TMarker, TValue> From<*mut TValue> for MutIdPtr<TMarker, TValue> {
    fn from(value: *mut TValue) -> Self {
        Self::from_mut_ptr(value)
    }
}

impl<TMarker, TValue> PartialEq for MutIdPtr<TMarker, TValue>
where
    *mut TValue: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.repr == other.repr
    }
}
