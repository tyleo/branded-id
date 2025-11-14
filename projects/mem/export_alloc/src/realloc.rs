use {
    safe_alloc::PowerOfTwoUsize,
    std::{
        alloc::{self, Layout},
        num::NonZeroUsize,
    },
};

/// Shrinks or grows a block of memory to the given `new_size` in bytes. The
/// block is described by the given `ptr` pointer and `layout`.
/// # Safety
/// The caller must ensure:
/// 1. `ptr` is allocated via this allocator,
/// 2. the layout is the same layout that was used to allocate that block of
///    memory,
/// 3. `new_size` is greater than zero.
/// 4. `new_size`, when rounded up to the nearest multiple of `layout.align()`,
///    does not overflow [`isize`] (i.e., the rounded value must be less than or
///    equal to [`isize::MAX`]).
///
/// If these are not followed, undefined behavior can result.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc(
    ptr: *mut u8,
    old_size: NonZeroUsize,
    old_align: PowerOfTwoUsize,
    new_size: NonZeroUsize,
) -> *mut u8 {
    let old_size = old_size.get();
    let old_align = old_align.as_usize();

    let old_layout = unsafe { Layout::from_size_align_unchecked(old_size, old_align) };
    let new_size = new_size.get();

    unsafe { alloc::realloc(ptr, old_layout, new_size) }
}
