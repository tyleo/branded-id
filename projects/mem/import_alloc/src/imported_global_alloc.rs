use {
    safe_alloc::PowerOfTwoUsize,
    std::{
        alloc::{GlobalAlloc, Layout},
        num::NonZeroUsize,
    },
};

/// A global allocator that uses imported allocation functions.
#[derive(Debug, Default, Clone, Copy)]
pub struct ImportedGlobalAlloc;

unsafe impl GlobalAlloc for ImportedGlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        unsafe {
            let size = NonZeroUsize::new_unchecked(size);
            let align = PowerOfTwoUsize::from_usize_unchecked(align);
            crate::alloc(size, align)
        }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        unsafe {
            let size = NonZeroUsize::new_unchecked(size);
            let align = PowerOfTwoUsize::from_usize_unchecked(align);
            crate::alloc_zeroed(size, align)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        let align = layout.align();

        unsafe {
            let size = NonZeroUsize::new_unchecked(size);
            let align = PowerOfTwoUsize::from_usize_unchecked(align);
            crate::dealloc(ptr, size, align)
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
        let old_size = old_layout.size();
        let old_align = old_layout.align();

        unsafe {
            let old_size = NonZeroUsize::new_unchecked(old_size);
            let old_align = PowerOfTwoUsize::from_usize_unchecked(old_align);
            let new_size = NonZeroUsize::new_unchecked(new_size);
            crate::realloc(ptr, old_size, old_align, new_size)
        }
    }
}
