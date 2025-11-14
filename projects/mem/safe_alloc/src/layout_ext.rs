use {
    crate::{PowerOfTwoUsize, Sealed},
    std::{alloc::Layout, mem::size_of},
};

pub trait LayoutExt: Sealed {
    /// Creates a new [`Layout`] from a size and power-of-two alignment.
    fn from_size_pow_2_align(size: usize, alignment: PowerOfTwoUsize) -> Option<Layout>;

    /// Creates a new [`Layout`] from a size and power-of-two alignment.
    /// # Safety
    /// `size`, when rounded up to the nearest multiple of `align`, must not
    /// overflow `isize` (i.e., the rounded value must be  less than or equal to
    /// `isize::MAX`).
    unsafe fn from_size_pow_2_align_unchecked(size: usize, alignment: PowerOfTwoUsize) -> Layout;

    /// Creates a new [`Layout`] from the type `T`.
    fn from_type<T>() -> Self
    where
        T: Sized;

    /// Converts the [`Layout`] into a size and alignment.
    fn into_size_align(self) -> (usize, PowerOfTwoUsize);

    /// Checks whether rounding `size` up to the nearest multiple of `align`
    /// would overflow `isize`.
    fn does_size_rounded_to_multiple_of_align_overflow_isize(
        size: usize,
        align: PowerOfTwoUsize,
    ) -> bool;
}

impl Sealed for Layout {}

impl LayoutExt for Layout {
    fn from_size_pow_2_align(size: usize, alignment: PowerOfTwoUsize) -> Option<Layout> {
        if Self::does_size_rounded_to_multiple_of_align_overflow_isize(size, alignment) {
            return None;
        }

        let result = unsafe { Layout::from_size_pow_2_align_unchecked(size, alignment) };
        Some(result)
    }

    unsafe fn from_size_pow_2_align_unchecked(size: usize, alignment: PowerOfTwoUsize) -> Layout {
        let alignment = alignment.as_usize();
        unsafe { Layout::from_size_align_unchecked(size, alignment) }
    }

    fn from_type<T>() -> Self {
        let size = size_of::<T>();
        let alignment = PowerOfTwoUsize::from_align_of::<T>();

        unsafe { Self::from_size_pow_2_align_unchecked(size, alignment) }
    }

    fn into_size_align(self) -> (usize, PowerOfTwoUsize) {
        let size = self.size();
        let alignment = unsafe { PowerOfTwoUsize::from_usize_unchecked(self.align()) };

        (size, alignment)
    }

    fn does_size_rounded_to_multiple_of_align_overflow_isize(
        size: usize,
        align: PowerOfTwoUsize,
    ) -> bool {
        size > max_size_for_align(align)
    }
}

#[inline(always)]
const fn max_size_for_align(align: PowerOfTwoUsize) -> usize {
    // (power-of-two implies align != 0.)

    // Rounded up size is:
    //   size_rounded_up = (size + align - 1) & !(align - 1);
    //
    // We know from above that align != 0. If adding (align - 1)
    // does not overflow, then rounding up will be fine.
    //
    // Conversely, &-masking with !(align - 1) will subtract off
    // only low-order-bits. Thus if overflow occurs with the sum,
    // the &-mask cannot subtract enough to undo that overflow.
    //
    // Above implies that checking for summation overflow is both
    // necessary and sufficient.

    // SAFETY: the maximum possible alignment is `isize::MAX + 1`,
    // so the subtraction cannot overflow.
    unsafe { usize::unchecked_sub(isize::MAX as usize + 1, align.as_usize()) }
}
