use crate::IsizeId;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct IdPtr<TMarker: ?Sized, TValue: ?Sized> {
    phantom: PhantomData<TMarker>,
    repr: *const TValue,
}

impl<TMarker, TValue> IdPtr<TMarker, TValue> {
    pub const fn from_ptr(repr: *const TValue) -> Self {
        Self {
            phantom: PhantomData,
            repr,
        }
    }

    /// # Safety
    /// This function should ensure that the offset results in a slice which
    /// shares memory with the original slice.
    pub const unsafe fn offset(self, offset: IsizeId<TMarker>) -> Self {
        let ptr = self.to_ptr().offset(offset.to_isize());
        Self::from_ptr(ptr)
    }

    pub const fn to_ptr(self) -> *const TValue {
        self.repr
    }
}

impl<TMarker, TValue> From<*const TValue> for IdPtr<TMarker, TValue> {
    fn from(value: *const TValue) -> Self {
        Self::from_ptr(value)
    }
}

impl<TMarker, TValue> From<*mut TValue> for IdPtr<TMarker, TValue> {
    fn from(value: *mut TValue) -> Self {
        Self::from_ptr(value)
    }
}
