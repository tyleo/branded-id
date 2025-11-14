use {
    safe_alloc::PowerOfTwoUsize,
    std::{
        alloc::{self, Layout},
        num::NonZeroUsize,
    },
};

/// Deallocates memory with the global allocator.
/// # Safety
/// The caller must ensure:
/// 1. `ptr` is a block of memory currently allocated via this allocator and,
/// 2. the layout is the same layout that was used to allocate that block of
///    memory.
///
/// Otherwise undefined behavior can result.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, size: NonZeroUsize, align: PowerOfTwoUsize) {
    let size = size.get();
    let align = align.as_usize();

    unsafe {
        let layout = Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout)
    }
}
