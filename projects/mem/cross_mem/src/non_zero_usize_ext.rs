use {
    crate::Sealed,
    std::{mem::size_of, num::NonZeroUsize},
};

pub trait NonZeroUsizeExt: Sealed {
    /// Creates a `NonZeroUsize` from the size of type `T`.
    fn from_size_of<T>() -> Option<NonZeroUsize>;
}

impl Sealed for NonZeroUsize {}

impl NonZeroUsizeExt for NonZeroUsize {
    fn from_size_of<T>() -> Option<NonZeroUsize> {
        let size = size_of::<T>();
        NonZeroUsize::new(size)
    }
}
