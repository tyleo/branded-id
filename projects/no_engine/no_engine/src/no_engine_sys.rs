use crate::{LogSys, NoEngineSysCtx, WindowSys};

pub struct NoEngineSys<Ctx: NoEngineSysCtx> {
    pub log_sys: LogSys<Ctx::LogSysCtx>,
    pub window_sys: WindowSys<Ctx::WindowSysCtx>,
}

impl<Ctx: NoEngineSysCtx> NoEngineSys<Ctx> {
    pub fn new(log_sys_ctx: Ctx::LogSysCtx, window_sys_ctx: Ctx::WindowSysCtx) -> Self {
        Self {
            log_sys: LogSys::new(log_sys_ctx),
            window_sys: WindowSys::new(window_sys_ctx),
        }
    }
}
