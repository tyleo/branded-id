use {bevy::prelude::*, no_engine_abstractions::LogSysCtx};

pub struct BevyLogSysCtx {}

impl BevyLogSysCtx {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BevyLogSysCtx {
    fn default() -> Self {
        Self::new()
    }
}

impl LogSysCtx for BevyLogSysCtx {
    fn log(&self, message: &str) {
        info!(message);
    }
}
