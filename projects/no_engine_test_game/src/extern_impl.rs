use {crate::run, no_engine_impl_extern::MutWindowCtxPtr};

#[unsafe(no_mangle)]
unsafe extern "C" fn run_with_extern(ctx: MutWindowCtxPtr) {
    run(ctx);
}
