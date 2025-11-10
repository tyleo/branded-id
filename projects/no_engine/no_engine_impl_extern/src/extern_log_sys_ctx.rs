use {no_engine_abstractions::LogSysCtx, std::ffi::c_void};

#[cfg_attr(feature = "wasm", link(wasm_import_module = "no_engine_extern_js"))]
#[cfg_attr(not(feature = "wasm"), link(name = "no_engine_extern"))]
unsafe extern "C" {
    fn log_sys_ctx_log(ctx_ptr: *mut c_void, message_ptr: *const u8, message_len: usize);
}

#[cfg_attr(feature = "wasm", link(wasm_import_module = "no_engine_extern_wasm"))]
#[cfg_attr(not(feature = "wasm"), link(name = "no_engine_extern"))]
unsafe extern "C" {
    fn log_sys_ctx_new() -> *mut c_void;

    fn log_sys_ctx_drop(ctx_ptr: *mut c_void);
}

#[repr(transparent)]
pub struct ExternLogSysCtx(*mut c_void);

impl ExternLogSysCtx {
    /// # Safety
    /// The `ptr` must point to a valid `LogSysCtx`.
    pub unsafe fn new() -> Self {
        unsafe { Self(log_sys_ctx_new()) }
    }
}

impl Drop for ExternLogSysCtx {
    fn drop(&mut self) {
        unsafe {
            log_sys_ctx_drop(self.0);
        }
    }
}

impl LogSysCtx for ExternLogSysCtx {
    fn log(&self, message: &str) {
        unsafe {
            log_sys_ctx_log(self.0, message.as_ptr(), message.len());
        }
    }
}
