use id_sys::UsizeId;

pub struct ColorU8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Vector2U32 {
    pub x: u32,
    pub y: u32,
}

pub struct MWindow;

pub trait WindowSysCtx {
    fn new() -> Self;

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
