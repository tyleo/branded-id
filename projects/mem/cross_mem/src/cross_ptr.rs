use {
    crate::{
        CrossMemError, CrossMemResult, CrossRef, CrossRefMut, CrossSafe, NonEmptyBytes,
        cross_alloc, cross_free, cross_memcpy,
    },
    safe_alloc::PowerOfTwoUsize,
    std::num::NonZeroUsize,
};

/// Cross-pointer type for cross-module memory allocations of memory.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CrossPtr {
    ptr: NonZeroUsize,
}

impl CrossPtr {
    /// Creates a new [`CrossPtr`] from a raw usize.
    pub fn alloc(bytes: NonZeroUsize, alignment: PowerOfTwoUsize) -> CrossMemResult<Self> {
        let result = unsafe { cross_alloc(bytes, alignment) };

        match NonZeroUsize::new(result) {
            Some(ptr) => Ok(CrossPtr { ptr }),
            None => Err(CrossMemError::AllocFailure),
        }
    }

    /// Creates [`CrossPtr`] that is dangling but non-null and well-aligned.
    /// This is mainly used for working with zero-sized types (ZSTs).
    pub fn dangling(alignment: PowerOfTwoUsize) -> Self {
        CrossPtr {
            ptr: alignment.get_non_zero_usize(),
        }
    }

    /// Frees memory in a separate module.
    /// # Safety
    /// `self` must have been allocated with [`CrossPtr::alloc`] with the same
    /// `bytes` and `alignment`.
    pub unsafe fn free(self, bytes: NonZeroUsize, alignment: PowerOfTwoUsize) {
        unsafe { cross_free(self, bytes, alignment) }
    }

    /// Creates a new [`CrossPtr`] from a byte slice by allocating memory
    /// in a separate module and copying the bytes over.
    pub fn from_non_empty_bytes(
        bytes: NonEmptyBytes,
        alignment: PowerOfTwoUsize,
    ) -> CrossMemResult<Self> {
        let len = bytes.len();

        let dest = Self::alloc(len, alignment)?;
        let src = bytes.as_bytes().as_ptr();
        let size = len.get();

        unsafe {
            cross_memcpy(dest, src, size);
        }

        Ok(dest)
    }

    /// Creates a new [`CrossPtr`] from an object by allocating memory in a
    /// separate module and copying the object's bytes over.
    pub fn from_object<T: CrossSafe>(object: &T) -> CrossMemResult<Self> {
        let align = PowerOfTwoUsize::from_align_of::<T>();
        match NonEmptyBytes::from_object(object) {
            Some(non_empty_bytes) => CrossPtr::from_non_empty_bytes(non_empty_bytes, align),
            None => Ok(CrossPtr::dangling(align)),
        }
    }

    /// Converts the [`CrossPtr`] back into an object by copying the bytes
    /// from the separate module into a newly created object.
    /// # Safety
    /// The [`CrossPtr`] must point to a valid object of type `T`.
    pub unsafe fn read<T: CrossSafe>(self) -> T {
        unsafe { self.as_ref().read::<T>() }
    }

    /// Writes an object into the memory pointed to by the [`CrossPtr`].
    /// # Safety
    /// The [`CrossPtr`] must point to a valid object of type `T`.
    pub unsafe fn write<T: CrossSafe>(&mut self, object: &T) {
        unsafe { self.as_mut().write(object) }
    }

    /// Returns the [`CrossPtr`] as a [`CrossRef`].
    /// # Safety
    /// The returned [`CrossRef`] cannot be aliased with a [`CrossRefMut`].
    pub unsafe fn as_ref<'a>(&'a self) -> CrossRef<'a> {
        unsafe { CrossRef::new(*self) }
    }

    /// Returns the [`CrossPtr`] as a [`CrossRefMut`].
    /// # Safety
    /// The returned [`CrossRefMut`] cannot be aliased with a [`CrossRef`] or
    /// another [`CrossRefMut`].
    pub unsafe fn as_mut<'a>(&'a mut self) -> CrossRefMut<'a> {
        unsafe { CrossRefMut::new(*self) }
    }
}

unsafe impl CrossSafe for CrossPtr {}
