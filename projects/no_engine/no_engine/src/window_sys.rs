use {
    crate::{ColorU8, MWindow, Vector2U32, WindowSysCtx},
    id_sys::{UsizeId, soa::UsizeIdStruct},
};

pub struct WindowSys<Ctx: WindowSysCtx> {
    ctx: Ctx,
    id_field: UsizeIdStruct<MWindow>,
}

impl<Ctx: WindowSysCtx> WindowSys<Ctx> {
    pub fn new(ctx: Ctx) -> Self {
        Self {
            ctx,
            id_field: UsizeIdStruct::new(),
        }
    }

    pub fn retain(&mut self, width: u32, height: u32) -> UsizeId<MWindow> {
        let id = self.id_field.retain();
        self.ctx.retain_window(id, width, height);
        id
    }

    /// # Safety
    /// The id must have been previously retained and not yet released.
    pub unsafe fn release(&mut self, id: UsizeId<MWindow>) {
        unsafe { self.ctx.release_window(id) }
        self.id_field.release(id);
    }

    /// # Safety
    /// The id must have been previously retained and not yet released.
    pub unsafe fn set_pixel_color(
        &mut self,
        id: UsizeId<MWindow>,
        position: Vector2U32,
        color: ColorU8,
    ) {
        unsafe {
            self.ctx.set_pixel_color(id, position, color);
        }
    }
}
