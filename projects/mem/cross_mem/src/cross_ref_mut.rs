use {
    crate::{CrossPtr, CrossSafe, cross_memcpy},
    std::{marker::PhantomData, mem::size_of},
};

/// Cross-reference type for cross-module memory allocations of mutable
/// memory.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CrossRefMut<'a> {
    ptr: CrossPtr,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> CrossRefMut<'a> {
    /// Creates a new [`CrossRefMut`] from a [`CrossPtr`].
    /// # Safety
    /// The [`CrossRefMut`] cannot be aliased with a [`crate::CrossRef`] or another
    /// [`CrossRefMut`].
    pub unsafe fn new(ptr: CrossPtr) -> Self {
        CrossRefMut {
            ptr,
            _lifetime: PhantomData,
        }
    }

    /// Writes an object into the memory pointed to by the [`CrossRefMut`].
    /// # Safety
    /// The [`CrossRefMut`] must point to a valid object of type `T`.
    pub unsafe fn write<T: CrossSafe>(&mut self, object: &T) {
        let dest = self.ptr;
        let src = object as *const T as *const u8;
        let size = size_of::<T>();

        unsafe {
            cross_memcpy(dest, src, size);
        }
    }
}
