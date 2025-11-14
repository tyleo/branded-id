use {crate::CrossPtr, safe_alloc::PowerOfTwoUsize, std::num::NonZeroUsize};

#[cfg_attr(feature = "wasm", link(wasm_import_module = "cross_mem_wasm"))]
#[cfg_attr(not(feature = "wasm"), link(name = "cross_mem"))]
unsafe extern "C" {
    /// Allocates memory in a separate module.
    /// # Safety
    /// `alignment` must be a power of two.
    pub(crate) unsafe fn cross_alloc(bytes: NonZeroUsize, alignment: PowerOfTwoUsize) -> usize;

    /// Frees memory in a separate module.
    /// /// # Safety
    /// `alignment` must be a power of two.
    pub(crate) unsafe fn cross_free(ptr: CrossPtr, bytes: NonZeroUsize, alignment: PowerOfTwoUsize);
}

#[cfg_attr(feature = "wasm", link(wasm_import_module = "cross_mem_js"))]
#[cfg_attr(not(feature = "wasm"), link(name = "cross_mem"))]
unsafe extern "C" {
    /// Copies memory from this module to another.
    /// # Safety
    /// `dest` and `src` must point to valid memory regions of at least `size`
    /// bytes.
    pub(crate) unsafe fn cross_memcpy(dest: CrossPtr, src: *const u8, size: usize);

    /// Copies memory from a separate module to this one.
    /// # Safety
    /// `dest` and `src` must point to valid memory regions of at least `size`
    /// bytes.
    pub(crate) unsafe fn cross_memcpy_back(dest: *mut u8, src: CrossPtr, size: usize);
}
