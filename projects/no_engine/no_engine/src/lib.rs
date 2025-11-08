use id_sys::{UsizeId, soa::UsizeIdStruct};

pub use no_engine_abstractions::*;

pub struct WindowSys {
    id_field: UsizeIdStruct<MWindow>,
}

impl WindowSys {
    pub fn new() -> Self {
        Self {
            id_field: UsizeIdStruct::new(),
        }
    }

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
        position: Vector2U32,
        color: ColorU8,
    ) {
        unsafe {
            ctx.set_pixel_color(id, position, color);
        }
    }
}

impl Default for WindowSys {
    fn default() -> Self {
        Self::new()
    }
}

// # Capabilities required for 1-D games:
// 1. Ability to create a 1xX window
// 2. Ability to color a pixel
// 3. Ability to handle touch input
// 4. Ability to handle keyboard input
// 5. Ability to load a sound effect
// 6. Ability to play a sound effect
