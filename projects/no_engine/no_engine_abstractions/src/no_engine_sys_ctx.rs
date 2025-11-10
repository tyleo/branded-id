use crate::{LogSysCtx, WindowSysCtx};

pub trait NoEngineSysCtx {
    type LogSysCtx: LogSysCtx;
    type WindowSysCtx: WindowSysCtx;
}
