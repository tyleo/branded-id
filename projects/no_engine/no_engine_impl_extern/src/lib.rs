use {
    id_sys::UsizeId,
    no_engine_abstractions::{ColorU8, MWindow, WindowSysCtx},
    std::ffi::c_void,
};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct MutWindowCtxPtr(pub *mut c_void);

#[cfg_attr(target_arch = "wasm32", link(wasm_import_module = "no_engine_extern"))]
#[cfg_attr(not(target_arch = "wasm32"), link(name = "no_engine_extern"))]
unsafe extern "C" {
    fn window_sys_ctx_retain_window(
        ctx_ptr: MutWindowCtxPtr,
        id: UsizeId<MWindow>,
        width: u32,
        height: u32,
    );

    fn window_sys_ctx_release_window(ctx_ptr: MutWindowCtxPtr, id: UsizeId<MWindow>);

    fn window_sys_ctx_set_pixel_color(
        ctx_ptr: MutWindowCtxPtr,
        id: UsizeId<MWindow>,
        x: u32,
        y: u32,
        color: ColorU8,
    );
}

impl WindowSysCtx for MutWindowCtxPtr {
    fn retain_window(&mut self, id: UsizeId<MWindow>, width: u32, height: u32) {
        unsafe { window_sys_ctx_retain_window(*self, id, width, height) }
    }

    unsafe fn release_window(&mut self, id: UsizeId<MWindow>) {
        unsafe { window_sys_ctx_release_window(*self, id) }
    }

    unsafe fn set_pixel_color(&mut self, id: UsizeId<MWindow>, x: u32, y: u32, color: ColorU8) {
        unsafe { window_sys_ctx_set_pixel_color(*self, id, x, y, color) }
    }
}
