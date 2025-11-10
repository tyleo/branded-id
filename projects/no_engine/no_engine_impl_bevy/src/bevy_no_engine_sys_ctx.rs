use {
    crate::{BevyLogSysCtx, BevyWindowSysCtx},
    no_engine_abstractions::NoEngineSysCtx,
};

pub struct BevyNoEngineSysCtx {
    pub log_sys_ctx: <BevyNoEngineSysCtx as NoEngineSysCtx>::LogSysCtx,
    pub window_sys_ctx: <BevyNoEngineSysCtx as NoEngineSysCtx>::WindowSysCtx,
}

impl BevyNoEngineSysCtx {
    pub fn new() -> Self {
        Self {
            log_sys_ctx: BevyLogSysCtx::new(),
            window_sys_ctx: BevyWindowSysCtx::new(),
        }
    }
}

impl Default for BevyNoEngineSysCtx {
    fn default() -> Self {
        Self::new()
    }
}

impl NoEngineSysCtx for BevyNoEngineSysCtx {
    type LogSysCtx = BevyLogSysCtx;
    type WindowSysCtx = BevyWindowSysCtx;
}
