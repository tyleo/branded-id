use {crate::get_extern_slice_mut, std::str::from_utf8_unchecked_mut};

/// # Safety
/// The `ptr` must point to a valid `str` with the specified length.
pub unsafe fn get_extern_str_mut<'a>(str_ptr: *mut u8, str_len: usize) -> &'a mut str {
    unsafe {
        let slice = get_extern_slice_mut::<u8>(str_ptr, str_len);
        from_utf8_unchecked_mut(slice)
    }
}
