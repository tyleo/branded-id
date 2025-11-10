use {crate::extern_impl::mem::get_extern_slice, std::str::from_utf8_unchecked};

/// # Safety
/// The `ptr` must point to a valid `str` with the specified length.
pub(crate) unsafe fn get_extern_str<'a>(str_ptr: *const u8, str_len: usize) -> &'a str {
    unsafe {
        let slice = get_extern_slice::<u8>(str_ptr, str_len);
        from_utf8_unchecked(slice)
    }
}
