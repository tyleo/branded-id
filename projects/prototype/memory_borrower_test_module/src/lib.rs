use import_alloc::*;

#[unsafe(no_mangle)]
pub extern "C" fn wasm_main() {
    let s = String::from("Hello from memory_borrower_test_module!");
    print!("{}", s);
}
