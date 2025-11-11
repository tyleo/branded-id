use {crate::get_extern_slice, std::str::from_utf8_unchecked};

/// # Safety
/// The `ptr` must point to a valid `str` with the specified length.
pub unsafe fn get_extern_str<'a>(str_ptr: *const u8, str_len: usize) -> &'a str {
    // Early return for empty strings to avoid unsafe calls.
    if str_len == 0 {
        return "";
    }

    unsafe {
        let slice = get_extern_slice::<u8>(str_ptr, str_len);
        from_utf8_unchecked(slice)
    }
}
