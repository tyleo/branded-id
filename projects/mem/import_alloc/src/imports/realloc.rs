use {safe_alloc::PowerOfTwoUsize, std::num::NonZeroUsize};

#[cfg_attr(feature = "wasm", link(wasm_import_module = "import_alloc"))]
#[cfg_attr(not(feature = "wasm"), link(name = "import_alloc"))]
unsafe extern "C" {
    /// Shrinks or grows a block of memory to the given `new_size` in bytes. The
    /// block is described by the given `ptr` pointer and `layout`.
    /// # Safety
    /// The caller must ensure:
    /// 1. `ptr` is allocated via this allocator,
    /// 2. the layout is the same layout that was used to allocate that block of
    ///    memory,
    /// 3. `new_size` is greater than zero.
    /// 4. `new_size`, when rounded up to the nearest multiple of `layout.align()`,
    ///    does not overflow [`isize`] (i.e., the rounded value must be less than or
    ///    equal to [`isize::MAX`]).
    ///
    /// If these are not followed, undefined behavior can result.
    pub unsafe fn realloc(
        ptr: *mut u8,
        old_size: NonZeroUsize,
        old_align: PowerOfTwoUsize,
        new_size: NonZeroUsize,
    ) -> *mut u8;
}
