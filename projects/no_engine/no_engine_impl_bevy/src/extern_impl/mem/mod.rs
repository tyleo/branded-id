mod exports;

mod get_extern;
mod get_extern_mut;
mod get_extern_slice;
mod get_extern_slice_mut;
mod get_extern_str;
mod get_extern_str_mut;

#[allow(unused_imports)]
pub(crate) use get_extern::*;
#[allow(unused_imports)]
pub(crate) use get_extern_mut::*;
pub(crate) use get_extern_slice::*;
pub(crate) use get_extern_slice_mut::*;
pub(crate) use get_extern_str::*;
#[allow(unused_imports)]
pub(crate) use get_extern_str_mut::*;
