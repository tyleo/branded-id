use std::slice::from_raw_parts;

/// # Safety
/// The `ptr` must point to a valid `[T]` slice with the specified length.
pub(crate) unsafe fn get_extern_slice<'a, T>(ptr: *const u8, length: usize) -> &'a [T] {
    unsafe { from_raw_parts(ptr as *const T, length) }
}
