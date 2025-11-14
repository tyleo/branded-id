/// # Safety
/// The `ptr` must point to a valid `T`.
pub unsafe fn get_extern_mut<'a, T>(ptr: *mut u8) -> &'a mut T {
    unsafe { &mut *(ptr as *mut T) }
}
