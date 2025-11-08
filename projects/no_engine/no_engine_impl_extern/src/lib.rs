use {
    id_sys::UsizeId,
    no_engine_abstractions::{ColorU8, MWindow, Vector2U32, WindowSysCtx},
    std::ffi::c_void,
};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct MutWindowCtxPtr(pub *mut c_void);

#[cfg_attr(feature = "wasm", link(wasm_import_module = "no_engine_extern",))]
#[cfg_attr(not(feature = "wasm"), link(name = "no_engine_extern"))]
unsafe extern "C" {
    fn window_sys_ctx_new() -> *mut c_void;

    fn window_sys_ctx_retain_window(
        ctx_ptr: *mut c_void,
        id: UsizeId<MWindow>,
        width: u32,
        height: u32,
    );

    fn window_sys_ctx_release_window(ctx_ptr: *mut c_void, id: UsizeId<MWindow>);

    fn window_sys_ctx_set_pixel_color(
        ctx_ptr: *mut c_void,
        id: UsizeId<MWindow>,
        x: u32,
        y: u32,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    );
}

impl WindowSysCtx for MutWindowCtxPtr {
    fn new() -> Self {
        let ctx_ptr = unsafe { window_sys_ctx_new() };
        MutWindowCtxPtr(ctx_ptr)
    }

    fn retain_window(&mut self, id: UsizeId<MWindow>, width: u32, height: u32) {
        unsafe { window_sys_ctx_retain_window(self.0, id, width, height) }
    }

    unsafe fn release_window(&mut self, id: UsizeId<MWindow>) {
        unsafe { window_sys_ctx_release_window(self.0, id) }
    }

    unsafe fn set_pixel_color(
        &mut self,
        id: UsizeId<MWindow>,
        position: Vector2U32,
        color: ColorU8,
    ) {
        unsafe {
            window_sys_ctx_set_pixel_color(
                self.0, id, position.x, position.y, color.r, color.g, color.b, color.a,
            )
        }
    }
}
