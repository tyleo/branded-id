use {
    crate::{ExternLogSysCtx, ExternWindowSysCtx},
    no_engine_abstractions::NoEngineSysCtx,
};

pub struct ExternNoEngineSysCtx {
    pub log_sys_ctx: <ExternNoEngineSysCtx as NoEngineSysCtx>::LogSysCtx,
    pub window_sys_ctx: <ExternNoEngineSysCtx as NoEngineSysCtx>::WindowSysCtx,
}

impl ExternNoEngineSysCtx {
    /// # Safety
    /// The functions linked into this crate must result in `new`
    /// implementations that result in valid instances.
    pub unsafe fn new() -> Self {
        unsafe {
            Self {
                log_sys_ctx: ExternLogSysCtx::new(),
                window_sys_ctx: ExternWindowSysCtx::new(),
            }
        }
    }
}

impl NoEngineSysCtx for ExternNoEngineSysCtx {
    type LogSysCtx = ExternLogSysCtx;
    type WindowSysCtx = ExternWindowSysCtx;
}
