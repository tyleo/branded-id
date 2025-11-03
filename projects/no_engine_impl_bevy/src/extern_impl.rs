use {
    crate::BevyWindowCtx,
    id_sys::UsizeId,
    no_engine_abstractions::{ColorU8, MWindow, WindowSysCtx},
    no_engine_impl_extern::MutWindowCtxPtr,
};

/// # Safety
/// The `ctx_ptr` must point to a valid `BevyWindowCtx`.
unsafe fn get_mut_bevy_window_ctx_from_ptr<'a>(ctx_ptr: MutWindowCtxPtr) -> &'a mut BevyWindowCtx {
    unsafe { &mut *(ctx_ptr.ptr as *mut BevyWindowCtx) }
}

/// # Safety
/// The `ctx_ptr` must point to a valid `BevyWindowCtx`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_window_sys_ctx_retain_window(
    ctx_ptr: MutWindowCtxPtr,
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
pub unsafe extern "C" fn bevy_window_sys_ctx_release_window(
    ctx_ptr: MutWindowCtxPtr,
    id: UsizeId<MWindow>,
) {
    let ctx = unsafe { get_mut_bevy_window_ctx_from_ptr(ctx_ptr) };
    unsafe { ctx.release_window(id) }
}

/// # Safety
/// The `ctx_ptr` must point to a valid `BevyWindowCtx`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_window_sys_ctx_set_pixel_color(
    ctx_ptr: MutWindowCtxPtr,
    id: UsizeId<MWindow>,
    x: u32,
    y: u32,
    color: ColorU8,
) {
    let ctx = unsafe { get_mut_bevy_window_ctx_from_ptr(ctx_ptr) };
    unsafe { ctx.set_pixel_color(id, x, y, color) }
}
