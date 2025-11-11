use std::alloc::{Layout, alloc, dealloc};

/// # Safety
/// `bytes` and `alignment` must be non-zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn extern_alloc(bytes: usize, alignment: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align_unchecked(bytes, alignment);
        alloc(layout)
    }
}

/// # Safety
/// `bytes` and `alignment` must be non-zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn extern_free(ptr: *mut u8, bytes: usize, alignment: usize) {
    unsafe {
        let layout = Layout::from_size_align_unchecked(bytes, alignment);
        dealloc(ptr, layout)
    }
}
