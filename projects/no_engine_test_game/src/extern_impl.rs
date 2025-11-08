use {crate::run, no_engine_impl_extern::MutWindowCtxPtr};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(not(feature = "wasm"), no_mangle)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub extern "C" fn run_with_extern() {
    println!("`run_with_extern`: called");
    run::<MutWindowCtxPtr>();
    println!("`run_with_extern`: completed");
}
