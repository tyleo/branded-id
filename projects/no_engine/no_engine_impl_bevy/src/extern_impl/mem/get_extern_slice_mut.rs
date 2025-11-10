use std::slice::from_raw_parts_mut;

/// # Safety
/// The `ptr` must point to a valid `[T]` slice with the specified length.
#[allow(dead_code)]
pub(crate) unsafe fn get_extern_slice_mut<'a, T>(ptr: *mut u8, length: usize) -> &'a mut [T] {
    unsafe { from_raw_parts_mut(ptr as *mut T, length) }
}
