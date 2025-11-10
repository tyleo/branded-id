use crate::LogSysCtx;

pub struct LogSys<Ctx: LogSysCtx> {
    ctx: Ctx,
}

impl<Ctx: LogSysCtx> LogSys<Ctx> {
    pub fn new(ctx: Ctx) -> Self {
        Self { ctx }
    }

    pub fn log(&self, message: &str) {
        self.ctx.log(message);
    }
}
