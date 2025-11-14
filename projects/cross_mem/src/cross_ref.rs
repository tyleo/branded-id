use {
    crate::{CrossPtr, CrossSafe, cross_memcpy_back},
    std::{marker::PhantomData, mem::size_of},
};

/// Cross-reference type for cross-module memory allocations of immutable
/// memory.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CrossRef<'a> {
    ptr: CrossPtr,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> CrossRef<'a> {
    /// Creates a new [`CrossRef`] from a [`CrossPtr`].
    /// # Safety
    /// The [`CrossRef`] cannot be aliased with a [`crate::CrossRefMut`].
    pub unsafe fn new(ptr: CrossPtr) -> Self {
        CrossRef {
            ptr,
            _lifetime: PhantomData,
        }
    }

    /// Converts the [`CrossRef`] back into an object by copying the bytes from
    /// the separate module into a newly created object.
    /// # Safety
    /// The [`CrossRef`] must point to a valid object of type `T`.
    pub unsafe fn read<T: CrossSafe>(&self) -> T {
        let mut object = std::mem::MaybeUninit::uninit();
        let dest = &mut object as *mut _ as *mut u8;
        let src = self.ptr;
        let size = size_of::<T>();

        unsafe {
            cross_memcpy_back(dest, src, size);
            object.assume_init()
        }
    }
}
