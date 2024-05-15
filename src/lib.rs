mod internal;
mod macros;

pub mod ext;

mod i32_id;
mod id_array;
mod id_ptr;
mod id_slice;
mod id_slice_index;
mod id_vec;
mod isize_id;
mod mut_id_ptr;
mod u32_id;
mod usize_id;

pub use i32_id::*;
pub use id_array::*;
pub use id_ptr::*;
pub use id_slice::*;
pub use id_slice_index::*;
pub use id_vec::*;
pub use isize_id::*;
pub use mut_id_ptr::*;
pub use u32_id::*;
pub use usize_id::*;

#[cfg(feature = "extends")]
pub mod extends;

#[cfg(feature = "soa")]
pub mod soa;

#[cfg(test)]
mod tests;
