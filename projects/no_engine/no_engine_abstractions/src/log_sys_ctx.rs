pub trait LogSysCtx {
    fn log(&self, message: &str);
}
