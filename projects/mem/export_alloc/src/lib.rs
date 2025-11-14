use {
    safe_alloc::{AllocSafeLayout, PowerOfTwoUsize},
    std::{alloc, num::NonZeroUsize},
};

/// Allocates memory with the global allocator.
#[unsafe(no_mangle)]
pub extern "C" fn alloc(size: NonZeroUsize, align: PowerOfTwoUsize) -> *mut u8 {
    let layout = AllocSafeLayout::from_size_align(size, align);
    let layout = *layout.as_layout();

    unsafe { alloc::alloc(layout) }
}

/// Allocates zero-initialized memory with the global allocator.
#[unsafe(no_mangle)]
pub extern "C" fn alloc_zeroed(size: NonZeroUsize, align: PowerOfTwoUsize) -> *mut u8 {
    let layout = AllocSafeLayout::from_size_align(size, align);
    let layout = *layout.as_layout();

    unsafe { alloc::alloc_zeroed(layout) }
}

/// Deallocates memory with the global allocator.
/// # Safety
/// The caller must ensure:
/// 1. `ptr`` is a block of memory currently allocated via this allocator and,
/// 2. the layout is the same layout that was used to allocate that block of
///    memory.
///
/// Otherwise undefined behavior can result.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, size: NonZeroUsize, align: PowerOfTwoUsize) {
    let layout = AllocSafeLayout::from_size_align(size, align);
    let layout = *layout.as_layout();

    unsafe { alloc::dealloc(ptr, layout) }
}

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
    let old_layout = AllocSafeLayout::from_size_align(old_size, old_align);
    let old_layout = *old_layout.as_layout();
    let new_size = new_size.get();

    unsafe { alloc::realloc(ptr, old_layout, new_size) }
}
