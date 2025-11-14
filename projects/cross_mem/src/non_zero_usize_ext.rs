use {
    crate::Sealed,
    std::{mem::size_of, num::NonZeroUsize},
};

pub trait NonZeroUsizeExt: Sealed {
    /// Creates a `NonZeroUsize` from the size of type `T`.
    fn from_size_of<T>() -> Self
    where
        T: Sized;
}

impl Sealed for NonZeroUsize {}

impl NonZeroUsizeExt for NonZeroUsize {
    fn from_size_of<T>() -> Self
    where
        T: Sized,
    {
        let size = size_of::<T>();
        unsafe { NonZeroUsize::new_unchecked(size) }
    }
}
