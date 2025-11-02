use {
    id_sys::{
        MutIdPtr, UsizeId,
        soa::{IdField, UsizeIdStruct},
    },
    std::ffi::c_void,
};

// No Engine Abstractions
// Backends and the library use this. Games use this as part of the library. Backends use this to
// fulfill contracts needed by the library. The library reexports this to provide functionality to
// games.

#[repr(C)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub struct MWindow;

pub trait WindowSysCtx {
    fn retain_window(&mut self, id: UsizeId<MWindow>, width: u32, height: u32);

    /// # Safety
    /// The id must have been previously retained and not yet released.
    unsafe fn release_window(&mut self, id: UsizeId<MWindow>);

    /// # Safety
    /// The id must have been previously retained and not yet released.
    unsafe fn set_pixel_color(&mut self, id: UsizeId<MWindow>, x: u32, y: u32, color: Color);
}

// No Engine Library
// Games use this to build gameplay logic.
// References: No Engine Abstractions

pub struct WindowSys {
    id_field: UsizeIdStruct<MWindow>,
}

impl WindowSys {
    pub fn retain<Ctx: WindowSysCtx>(
        &mut self,
        ctx: &mut Ctx,
        width: u32,
        height: u32,
    ) -> UsizeId<MWindow> {
        let id = self.id_field.retain();
        ctx.retain_window(id, width, height);
        id
    }

    /// # Safety
    /// The id must have been previously retained and not yet released.
    pub unsafe fn release<Ctx: WindowSysCtx>(&mut self, ctx: &mut Ctx, id: UsizeId<MWindow>) {
        unsafe { ctx.release_window(id) }
        self.id_field.release(id);
    }

    /// # Safety
    /// The id must have been previously retained and not yet released.
    pub unsafe fn set_pixel_color<Ctx: WindowSysCtx>(
        &mut self,
        ctx: &mut Ctx,
        id: UsizeId<MWindow>,
        x: u32,
        y: u32,
        color: Color,
    ) {
        unsafe {
            ctx.set_pixel_color(id, x, y, color);
        }
    }
}

// No Engine FFI Abstractions
// A implementation of the abstractions which can be passed across FFI boundaries. This can be
// statically linked in to support multiple backends without needing to recompile the game.
// References: No Engine Abstractions

pub struct MWindowCtx;

pub type MutWindowCtxPtr = MutIdPtr<MWindowCtx, c_void>;

#[cfg_attr(target_arch = "wasm32", link(wasm_import_module = "no_engine_ffi"))]
#[cfg_attr(not(target_arch = "wasm32"), link(name = "no_engine_ffi"))]
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
        color: Color,
    );
}

impl WindowSysCtx for MutWindowCtxPtr {
    fn retain_window(&mut self, id: UsizeId<MWindow>, width: u32, height: u32) {
        unsafe { window_sys_ctx_retain_window(*self, id, width, height) }
    }

    unsafe fn release_window(&mut self, id: UsizeId<MWindow>) {
        unsafe { window_sys_ctx_release_window(*self, id) }
    }

    unsafe fn set_pixel_color(&mut self, id: UsizeId<MWindow>, x: u32, y: u32, color: Color) {
        unsafe { window_sys_ctx_set_pixel_color(*self, id, x, y, color) }
    }
}

// Bevy Impl
// References: Bevy; No Engine Abstractions
// Conditionally References: No Engine FFI Abstractions

pub struct BevyWindowCtx {
    windows: IdField<MWindow, ()>,
}

impl BevyWindowCtx {
    pub fn new() -> Self {
        Self {
            windows: IdField::new(),
        }
    }
}

impl Default for BevyWindowCtx {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowSysCtx for BevyWindowCtx {
    fn retain_window(&mut self, id: UsizeId<MWindow>, _width: u32, _height: u32) {
        self.windows.retain(id, ());
    }

    unsafe fn release_window(&mut self, id: UsizeId<MWindow>) {
        unsafe { self.windows.release(id) }
    }

    unsafe fn set_pixel_color(&mut self, _id: UsizeId<MWindow>, _x: u32, _y: u32, _color: Color) {
        // Implementation to set pixel color in Bevy window
    }
}

//#[cfg(feature = "ffi")]
unsafe fn get_mut_bevy_window_ctx_from_ptr<'a>(ctx_ptr: MutWindowCtxPtr) -> &'a mut BevyWindowCtx {
    unsafe { &mut *(ctx_ptr.to_mut_ptr() as *mut BevyWindowCtx) }
}

/// # Safety
/// The `ctx_ptr` must point to a valid `BevyWindowCtx`.
// #[cfg(feature = "ffi")]
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
// #[cfg(feature = "ffi")]
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
//#[cfg(feature = "ffi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bevy_window_sys_ctx_set_pixel_color(
    ctx_ptr: MutWindowCtxPtr,
    id: UsizeId<MWindow>,
    x: u32,
    y: u32,
    color: Color,
) {
    let ctx = unsafe { get_mut_bevy_window_ctx_from_ptr(ctx_ptr) };
    unsafe { ctx.set_pixel_color(id, x, y, color) }
}

// Game
// References: No Engine Abstractions, No Engine Library
// Conditionally References: No Engine FFI Abstractions

fn run<Ctx: WindowSysCtx>(mut ctx: Ctx) {
    let mut window_sys = WindowSys {
        id_field: UsizeIdStruct::new(),
    };

    let window_id = window_sys.retain(&mut ctx, 10, 1);

    for x in 1..10 {
        let color = if (x % 2) == 0 {
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }
        } else {
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            }
        };
        unsafe { window_sys.set_pixel_color(&mut ctx, window_id, x, 0, color) }
    }

    unsafe {
        window_sys.release(&mut ctx, window_id);
    }
}

// #[cfg(feature = "ffi")]
#[unsafe(no_mangle)]
unsafe extern "C" fn run_with_ffi(ctx: MutWindowCtxPtr) {
    run(ctx);
}

// # Capabilities required for 1-D games:
// 1. Ability to create a 1xX window
// 2. Ability to color a pixel
// 3. Ability to handle touch input
// 4. Ability to handle keyboard input
// 5. Ability to load a sound effect
// 6. Ability to play a sound effect
