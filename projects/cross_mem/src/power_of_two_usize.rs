use std::num::NonZeroUsize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PowerOfTwoUsize(NonZeroUsize);

const fn is_power_of_two(x: usize) -> bool {
    x != 0 && (x & (x - 1)) == 0
}

impl PowerOfTwoUsize {
    /// Creates a new `PowerOfTwoUsize` if `value` is a power of two.
    pub const fn from_usize(value: usize) -> Option<Self> {
        if is_power_of_two(value) {
            Some(unsafe { PowerOfTwoUsize::from_usize_unchecked(value) })
        } else {
            None
        }
    }

    /// Creates a new `PowerOfTwoUsize` without checking if `value` is a power
    /// of two.
    /// # Safety
    /// The caller must ensure that `value` is a power of two.
    pub const unsafe fn from_usize_unchecked(value: usize) -> Self {
        unsafe {
            PowerOfTwoUsize::from_non_zero_usize_unchecked(NonZeroUsize::new_unchecked(value))
        }
    }

    /// Creates a new `PowerOfTwoUsize` if `value` is a power of two.
    pub const fn from_non_zero_usize(value: NonZeroUsize) -> Option<Self> {
        Self::from_usize(value.get())
    }

    /// Creates a new `PowerOfTwoUsize` without checking if `value` is a power
    /// of two.
    /// # Safety
    /// The caller must ensure that `value` is a power of two.
    pub const unsafe fn from_non_zero_usize_unchecked(value: NonZeroUsize) -> Self {
        PowerOfTwoUsize(value)
    }

    /// Creates a new `PowerOfTwoUsize` from the alignment of type `T`.
    pub const fn from_align_of<T>() -> Self {
        let align = std::mem::align_of::<T>();
        unsafe { PowerOfTwoUsize::from_usize_unchecked(align) }
    }

    /// Returns the underlying `NonZeroUsize` value.
    pub const fn get_non_zero_usize(&self) -> NonZeroUsize {
        self.0
    }

    /// Returns the underlying `usize` value.
    pub const fn get_usize(&self) -> usize {
        self.get_non_zero_usize().get()
    }
}

impl TryFrom<usize> for PowerOfTwoUsize {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::from_usize(value).ok_or(())
    }
}

impl TryFrom<NonZeroUsize> for PowerOfTwoUsize {
    type Error = ();

    fn try_from(value: NonZeroUsize) -> Result<Self, Self::Error> {
        Self::from_non_zero_usize(value).ok_or(())
    }
}
