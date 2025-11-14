use {
    crate::{PowerOfTwoUsize, Sealed},
    std::{alloc::Layout, mem::size_of},
};

pub trait LayoutExt: Sealed {
    /// Creates a new `Self` from a size and alignment.
    fn from_size_align_safe(size: usize, alignment: PowerOfTwoUsize) -> Self;

    /// Creates a new `Self` from the type `T`.
    fn from_type<T>() -> Self
    where
        T: Sized;

    /// Converts the layout into a size and alignment.
    fn into_size_align(self) -> (usize, PowerOfTwoUsize);
}

impl Sealed for Layout {}

impl LayoutExt for Layout {
    fn from_size_align_safe(size: usize, alignment: PowerOfTwoUsize) -> Self {
        unsafe { Layout::from_size_align_unchecked(size, alignment.get_usize()) }
    }

    fn from_type<T>() -> Self {
        let size = size_of::<T>();
        let alignment = PowerOfTwoUsize::from_align_of::<T>();

        Self::from_size_align_safe(size, alignment)
    }

    fn into_size_align(self) -> (usize, PowerOfTwoUsize) {
        let size = self.size();
        let alignment = unsafe { PowerOfTwoUsize::from_usize_unchecked(self.align()) };

        (size, alignment)
    }
}
