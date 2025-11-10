use std::ffi::c_void;

/// # Safety
/// The `ctx_ptr` must point to a valid `TCtx`.
pub(crate) unsafe fn ptr_to_ctx<TCtx>(ctx_ptr: *const c_void) -> &'static TCtx {
    unsafe { &*(ctx_ptr as *const TCtx) }
}
