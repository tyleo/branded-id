use no_engine::*;

#[cfg(feature = "extern")]
pub mod extern_impl;

fn run<Ctx: WindowSysCtx>() {
    println!("`run`: called");
    println!("`run`: creating ctx");
    let mut ctx = Ctx::new();

    println!("`run`: creating window_sys");
    let mut window_sys = WindowSys::new();

    println!("`run`: retaining window");
    let window_id = window_sys.retain(&mut ctx, 10, 1);

    println!("`run`: coloring window");
    for x in 1..10 {
        let color = if (x % 2) == 0 {
            ColorU8 {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }
        } else {
            ColorU8 {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            }
        };
        unsafe { window_sys.set_pixel_color(&mut ctx, window_id, x, 0, color) }
    }

    println!("`run`: releasing window");
    unsafe {
        window_sys.release(&mut ctx, window_id);
    }
    println!("`run`: completed");
}
