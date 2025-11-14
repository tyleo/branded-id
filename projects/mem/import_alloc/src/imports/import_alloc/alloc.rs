use {safe_alloc::PowerOfTwoUsize, std::num::NonZeroUsize};

#[cfg_attr(feature = "wasm", link(wasm_import_module = "import_alloc"))]
#[cfg_attr(not(feature = "wasm"), link(name = "import_alloc"))]
unsafe extern "C" {
    /// Allocates memory with the global allocator.
    pub fn alloc(size: NonZeroUsize, align: PowerOfTwoUsize) -> *mut u8;
}
