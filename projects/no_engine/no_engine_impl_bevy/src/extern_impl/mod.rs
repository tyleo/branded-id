pub mod ptr_to_ctx;
pub mod ptr_to_ctx_mut;

mod exports;

pub(crate) use ptr_to_ctx::*;
pub(crate) use ptr_to_ctx_mut::*;

use wasm_bindgen::prelude::wasm_bindgen;

// This is required to make wasm-bindgen work properly.
#[wasm_bindgen]
extern "C" {}
