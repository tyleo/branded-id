use {crate::run, no_engine_impl_extern::MutWindowCtxPtr};

#[unsafe(no_mangle)]
pub extern "C" fn run_with_extern() {
    println!("`run_with_extern`: called");
    run::<MutWindowCtxPtr>();
    println!("`run_with_extern`: completed");
}
