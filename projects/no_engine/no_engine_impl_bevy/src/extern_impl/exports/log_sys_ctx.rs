use {
    crate::{
        BevyLogSysCtx,
        extern_impl::{mem, ptr_to_ctx},
    },
    no_engine_abstractions::LogSysCtx,
    std::ffi::c_void,
};

/// # Safety
/// The `ctx_ptr` must point to a valid `BevyLogSysCtx`.
unsafe fn get_bevy_log_sys_ctx_from_ptr<'a>(ctx_ptr: *const c_void) -> &'a BevyLogSysCtx {
    unsafe { ptr_to_ctx(ctx_ptr) }
}

#[unsafe(no_mangle)]
pub extern "C" fn log_sys_ctx_new() -> *mut c_void {
    let ctx = BevyLogSysCtx::new();
    let boxed_ctx = Box::new(ctx);
    Box::into_raw(boxed_ctx) as *mut c_void
}

/// # Safety
/// The `ctx_ptr` must point to a valid `BevyNoEngineCtx`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_sys_ctx_drop(ctx_ptr: *mut c_void) {
    let ctx_ptr = ctx_ptr as *mut BevyLogSysCtx;
    unsafe { drop(Box::from_raw(ctx_ptr)) }
}

/// # Safety
/// The `ctx_ptr` must point to a valid `BevyLogSysCtx`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_sys_ctx_log(ctx_ptr: *mut c_void, str_ptr: *const u8, str_len: usize) {
    let ctx = unsafe { get_bevy_log_sys_ctx_from_ptr(ctx_ptr) };
    let message = unsafe { mem::get_extern_str(str_ptr, str_len) };
    ctx.log(message);
}
