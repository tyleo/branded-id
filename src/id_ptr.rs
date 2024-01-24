use crate::IsizeId;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct IdPtr<TMarker: ?Sized, TValue: ?Sized> {
    phantom: PhantomData<TMarker>,
    repr: *const TValue,
}

impl<TMarker, TValue> IdPtr<TMarker, TValue> {
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
    /// See https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/primitive.pointer.html#safety-1
    pub const unsafe fn offset(self, offset: IsizeId<TMarker>) -> Self {
        let ptr = self.to_ptr().offset(offset.to_isize());
        Self::from_ptr(ptr)
    }

    pub const fn to_ptr(self) -> *const TValue {
        self.repr
    }
}

impl<TMarker, TValueA> Eq for IdPtr<TMarker, TValueA> where *const TValueA: PartialEq {}

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

impl<TMarker, TValueA, TValueB> PartialEq<IdPtr<TMarker, TValueB>> for IdPtr<TMarker, TValueA>
where
    *const TValueA: PartialEq<*const TValueB>,
{
    fn eq(&self, other: &IdPtr<TMarker, TValueB>) -> bool {
        self.repr.eq(&other.repr)
    }
}
