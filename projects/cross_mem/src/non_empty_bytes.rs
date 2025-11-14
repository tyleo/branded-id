use {
    crate::CrossSafe,
    std::{mem::size_of, num::NonZeroUsize, slice},
};

/// A wrapper around a non-empty byte slice.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonEmptyBytes<'a> {
    bytes: &'a [u8],
}

impl<'a> NonEmptyBytes<'a> {
    /// Creates a new `NonEmptyBytes` from a byte slice.
    /// # Safety
    /// `bytes` must be non-empty.
    pub const fn from_bytes(bytes: &'a [u8]) -> Option<NonEmptyBytes<'a>> {
        if bytes.is_empty() {
            return None;
        }
        Some(NonEmptyBytes { bytes })
    }

    /// Creates a new `NonEmptyBytes` from an object.
    pub const fn from_object<T: CrossSafe>(object: &'a T) -> Option<NonEmptyBytes<'a>> {
        let data = object as *const T as *const u8;
        let size = size_of::<T>();

        let bytes = unsafe { slice::from_raw_parts(data, size) };

        Self::from_bytes(bytes)
    }

    /// Returns the length of the byte slice.
    pub const fn len(&self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(self.bytes.len()) }
    }

    /// Returns the inner byte slice.
    pub const fn as_bytes(&self) -> &[u8] {
        self.bytes
    }
}
