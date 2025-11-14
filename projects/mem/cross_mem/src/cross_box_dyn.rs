use {
    crate::{
        CrossMemError, CrossMemResult, CrossPtr, CrossRef, CrossRefMut, CrossSafe, NonEmptyBytes,
    },
    safe_alloc::{LayoutExt, PowerOfTwoUsize},
    std::{alloc::Layout, num::NonZeroUsize},
};

/// Cross-pointer type for cross-module memory allocations with automatic
/// cleanup. This type can own heap allocations whose size is only known at
/// runtime (e.g., arbitrary byte buffers, FFI structs, etc.).
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CrossBoxDyn {
    ptr: CrossPtr,
    layout: Layout,
}

impl Drop for CrossBoxDyn {
    fn drop(&mut self) {
        let (bytes, alignment) = self.layout.into_size_align();

        // Deal with zero-sized allocations properly.
        if let Some(bytes) = NonZeroUsize::new(bytes) {
            let ptr = self.ptr;
            unsafe { ptr.free(bytes, alignment) }
        }
    }
}

impl CrossBoxDyn {
    /// Allocates a new uninitialized [`CrossBoxDyn`] with the given size and
    /// alignment.
    pub fn alloc(bytes: usize, alignment: PowerOfTwoUsize) -> CrossMemResult<Self> {
        // Deal with zero-sized allocations properly.
        let ptr = match NonZeroUsize::new(bytes) {
            Some(bytes) => CrossPtr::alloc(bytes, alignment)?,
            None => CrossPtr::dangling(alignment),
        };

        let layout =
            Layout::from_size_pow_2_align(bytes, alignment).ok_or(CrossMemError::SizeOverflow)?;

        Ok(CrossBoxDyn { ptr, layout })
    }

    /// Creates a new [`CrossBoxDyn`] from a byte slice.
    pub fn from_non_empty_bytes(
        bytes: NonEmptyBytes,
        alignment: PowerOfTwoUsize,
    ) -> CrossMemResult<Self> {
        let ptr = CrossPtr::from_non_empty_bytes(bytes, alignment)?;
        let layout = Layout::from_size_pow_2_align(bytes.len().get(), alignment)
            .ok_or(CrossMemError::SizeOverflow)?;

        Ok(CrossBoxDyn { ptr, layout })
    }

    /// Creates a new [`CrossBoxDyn`] from an object.
    pub fn from_object<T: CrossSafe + Sized>(object: &T) -> CrossMemResult<Self> {
        let ptr = CrossPtr::from_object(object)?;
        let layout = Layout::from_type::<T>();

        Ok(CrossBoxDyn { ptr, layout })
    }

    /// Reads an object from the memory pointed to by the [`CrossBoxDyn`].
    /// # Safety
    /// The [`CrossBoxDyn`] must point to a valid object of type `T`.
    pub unsafe fn read<T: CrossSafe>(&self) -> T {
        unsafe { self.as_ref().read::<T>() }
    }

    /// Writes an object into the memory pointed to by the [`CrossBoxDyn`].
    /// # Safety
    /// The [`CrossBoxDyn`] must point to a valid object of type `T`.
    pub unsafe fn write<T: CrossSafe>(&mut self, object: &T) {
        unsafe { self.as_mut().write(object) }
    }

    /// Returns the inner [`CrossPtr`] as a [`CrossRef`].
    pub fn as_ref<'a>(&'a self) -> CrossRef<'a> {
        unsafe { self.ptr.as_ref() }
    }

    /// Returns the inner [`CrossPtr`] as a [`CrossRefMut`].
    pub fn as_mut<'a>(&'a mut self) -> CrossRefMut<'a> {
        unsafe { self.ptr.as_mut() }
    }
}
