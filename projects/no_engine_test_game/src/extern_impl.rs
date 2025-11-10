use {crate::run, no_engine::NoEngineSys, no_engine_impl_extern::ExternNoEngineSysCtx};

/// # Safety
/// The functions linked into this crate must result in `new` implementations
/// that result in valid instances of `ExternNoEngineSysCtx`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_with_extern() {
    let ctx = unsafe { ExternNoEngineSysCtx::new() };
    let no_engine_sys =
        NoEngineSys::<ExternNoEngineSysCtx>::new(ctx.log_sys_ctx, ctx.window_sys_ctx);
    run(no_engine_sys);
}
