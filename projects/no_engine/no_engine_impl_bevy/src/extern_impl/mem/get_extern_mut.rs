/// # Safety
/// The `ptr` must point to a valid `T`.
#[allow(dead_code)]
pub(crate) unsafe fn get_extern_mut<'a, T>(ptr: *mut u8) -> &'a mut T {
    unsafe { &mut *(ptr as *mut T) }
}
