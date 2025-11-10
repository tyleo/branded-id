use std::ffi::c_void;

/// # Safety
/// The `ctx_ptr` must point to a valid `TCtx`.
pub(crate) unsafe fn ptr_to_ctx_mut<TCtx>(ctx_ptr: *mut c_void) -> &'static mut TCtx {
    unsafe { &mut *(ctx_ptr as *mut TCtx) }
}
