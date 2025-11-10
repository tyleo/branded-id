use no_engine::*;

#[cfg(feature = "extern")]
pub mod extern_impl;

fn run<Ctx: NoEngineSysCtx>(no_engine_sys: NoEngineSys<Ctx>) {
    let log_sys = no_engine_sys.log_sys;
    let mut window_sys = no_engine_sys.window_sys;

    log_sys.log("`run`: called");

    log_sys.log("`run`: retaining window");
    let window_id = window_sys.retain(10, 1);

    log_sys.log("`run`: coloring window");
    for x in 1..10 {
        let position = Vector2U32 { x, y: 0 };
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
        unsafe { window_sys.set_pixel_color(window_id, position, color) }
    }

    log_sys.log("`run`: releasing window");
    unsafe {
        window_sys.release(window_id);
    }
    log_sys.log("`run`: completed");
}
