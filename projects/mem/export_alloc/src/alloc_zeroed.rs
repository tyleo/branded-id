use {
    safe_alloc::PowerOfTwoUsize,
    std::{
        alloc::{self, Layout},
        num::NonZeroUsize,
    },
};

/// Allocates zero-initialized memory with the global allocator.
/// # Safety
/// `size`, when rounded up to the nearest multiple of `align`,
///  must not overflow `isize` (i.e., the rounded value must be
///  less than or equal to `isize::MAX`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn alloc_zeroed(size: NonZeroUsize, align: PowerOfTwoUsize) -> *mut u8 {
    let size = size.get();
    let align = align.as_usize();

    unsafe {
        let layout = Layout::from_size_align_unchecked(size, align);
        alloc::alloc_zeroed(layout)
    }
}
