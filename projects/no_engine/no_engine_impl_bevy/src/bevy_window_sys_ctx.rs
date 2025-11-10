use {
    id_sys::{UsizeId, soa::IdField},
    no_engine_abstractions::{ColorU8, MWindow, Vector2U32, WindowSysCtx},
};

pub struct BevyWindowSysCtx {
    windows: IdField<MWindow, ()>,
}

impl BevyWindowSysCtx {
    pub fn new() -> Self {
        Self {
            windows: IdField::new(),
        }
    }
}

impl Default for BevyWindowSysCtx {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowSysCtx for BevyWindowSysCtx {
    fn retain_window(&mut self, id: UsizeId<MWindow>, _width: u32, _height: u32) {
        self.windows.retain(id, ());
    }

    unsafe fn release_window(&mut self, id: UsizeId<MWindow>) {
        unsafe { self.windows.release(id) }
    }

    unsafe fn set_pixel_color(
        &mut self,
        _id: UsizeId<MWindow>,
        _position: Vector2U32,
        _color: ColorU8,
    ) {
        // Implementation to set pixel color in Bevy window
    }
}
