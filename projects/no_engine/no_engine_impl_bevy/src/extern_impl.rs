use {
    crate::BevyWindowCtx,
    id_sys::UsizeId,
    no_engine_abstractions::{ColorU8, MWindow, Vector2U32, WindowSysCtx},
    std::ffi::c_void,
};

/// # Safety
/// The `ctx_ptr` must point to a valid `BevyWindowCtx`.
unsafe fn get_mut_bevy_window_ctx_from_ptr<'a>(ctx_ptr: *mut c_void) -> &'a mut BevyWindowCtx {
    unsafe { &mut *(ctx_ptr as *mut BevyWindowCtx) }
}

#[unsafe(no_mangle)]
pub extern "C" fn window_sys_ctx_new() -> *mut c_void {
    let ctx = BevyWindowCtx::new();
    let boxed_ctx = Box::new(ctx);
    Box::into_raw(boxed_ctx) as *mut c_void
}

/// # Safety
/// The `ctx_ptr` must point to a valid `BevyWindowCtx`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_sys_ctx_drop(ctx_ptr: *mut c_void) {
    let ctx_ptr = ctx_ptr as *mut BevyWindowCtx;
    unsafe { drop(Box::from_raw(ctx_ptr)) }
}

/// # Safety
/// The `ctx_ptr` must point to a valid `BevyWindowCtx`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_sys_ctx_retain_window(
    ctx_ptr: *mut c_void,
    id: UsizeId<MWindow>,
    width: u32,
    height: u32,
) {
    let ctx = unsafe { get_mut_bevy_window_ctx_from_ptr(ctx_ptr) };
    ctx.retain_window(id, width, height);
}

/// # Safety
/// The `ctx_ptr` must point to a valid `BevyWindowCtx`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_sys_ctx_release_window(ctx_ptr: *mut c_void, id: UsizeId<MWindow>) {
    let ctx = unsafe { get_mut_bevy_window_ctx_from_ptr(ctx_ptr) };
    unsafe { ctx.release_window(id) }
}

/// # Safety
/// The `ctx_ptr` must point to a valid `BevyWindowCtx`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_sys_ctx_set_pixel_color(
    ctx_ptr: *mut c_void,
    id: UsizeId<MWindow>,
    x: u32,
    y: u32,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    let ctx = unsafe { get_mut_bevy_window_ctx_from_ptr(ctx_ptr) };
    unsafe { ctx.set_pixel_color(id, Vector2U32 { x, y }, ColorU8 { r, g, b, a }) }
}
