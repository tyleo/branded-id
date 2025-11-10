/// # Safety
/// The `ptr` must point to a valid `T`.
#[allow(dead_code)]
pub(crate) unsafe fn get_extern<'a, T>(ptr: *const u8) -> &'a T {
    unsafe { &*(ptr as *const T) }
}
