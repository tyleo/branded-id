use {
    crate::{ColorU8, MWindow, Vector2U32},
    id_sys::UsizeId,
};

pub trait WindowSysCtx {
    fn retain_window(&mut self, id: UsizeId<MWindow>, width: u32, height: u32);

    /// # Safety
    /// The id must have been previously retained and not yet released.
    unsafe fn release_window(&mut self, id: UsizeId<MWindow>);

    /// # Safety
    /// The id must have been previously retained and not yet released.
    unsafe fn set_pixel_color(
        &mut self,
        id: UsizeId<MWindow>,
        position: Vector2U32,
        color: ColorU8,
    );
}
