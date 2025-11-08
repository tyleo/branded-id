use {
    id_sys::{UsizeId, soa::IdField},
    no_engine_abstractions::{ColorU8, MWindow, WindowSysCtx},
};

#[cfg(feature = "extern")]
pub mod extern_impl;

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
    fn new() -> Self {
        Self::default()
    }

    fn retain_window(&mut self, id: UsizeId<MWindow>, _width: u32, _height: u32) {
        self.windows.retain(id, ());
    }

    unsafe fn release_window(&mut self, id: UsizeId<MWindow>) {
        unsafe { self.windows.release(id) }
    }

    unsafe fn set_pixel_color(&mut self, _id: UsizeId<MWindow>, _x: u32, _y: u32, _color: ColorU8) {
        // Implementation to set pixel color in Bevy window
    }
}
