use {
    id_sys::UsizeId,
    no_engine_abstractions::{ColorU8, MWindow, Vector2U32, WindowSysCtx},
    std::ffi::c_void,
};

#[cfg_attr(feature = "wasm", link(wasm_import_module = "no_engine_extern_wasm"))]
#[cfg_attr(not(feature = "wasm"), link(name = "no_engine_extern"))]
unsafe extern "C" {
    fn window_sys_ctx_new() -> *mut c_void;

    fn window_sys_ctx_drop(ctx_ptr: *mut c_void);

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

#[repr(transparent)]
pub struct ExternWindowSysCtx(*mut c_void);

impl ExternWindowSysCtx {
    /// # Safety
    /// The value returned by `window_sys_ctx_new` must point to a valid
    /// `WindowSysCtx`.
    pub unsafe fn new() -> Self {
        unsafe { Self(window_sys_ctx_new()) }
    }
}

impl Drop for ExternWindowSysCtx {
    fn drop(&mut self) {
        unsafe {
            window_sys_ctx_drop(self.0);
        }
    }
}

impl WindowSysCtx for ExternWindowSysCtx {
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
