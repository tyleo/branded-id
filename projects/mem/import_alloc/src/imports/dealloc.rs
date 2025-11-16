use {safe_alloc::PowerOfTwoUsize, std::num::NonZeroUsize};

#[cfg_attr(feature = "wasm", link(wasm_import_module = "import_alloc"))]
#[cfg_attr(not(feature = "wasm"), link(name = "import_alloc"))]
unsafe extern "C" {
    /// Deallocates memory with the global allocator.
    /// # Safety
    /// The caller must ensure:
    /// 1. `ptr` is a block of memory currently allocated via this allocator
    ///    and,
    /// 2. the layout is the same layout that was used to allocate that block of
    ///    memory.
    ///
    /// Otherwise undefined behavior can result.
    pub fn dealloc(ptr: *mut u8, size: NonZeroUsize, align: PowerOfTwoUsize);
}
