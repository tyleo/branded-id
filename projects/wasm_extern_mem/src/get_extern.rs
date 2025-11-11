/// # Safety
/// The `ptr` must point to a valid `T`.
pub unsafe fn get_extern<'a, T>(ptr: *const u8) -> &'a T {
    unsafe { &*(ptr as *const T) }
}
