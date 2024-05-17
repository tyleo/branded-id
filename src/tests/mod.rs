pub mod util;

mod ext;
mod i32_id_tests;
mod id_array_tests;
mod id_ptr_tests;
mod id_slice_tests;
mod id_vec_tests;
mod isize_id_tests;
mod macros;
mod mut_id_ptr_tests;
mod u32_id_tests;
mod usize_id_tests;

#[cfg(feature = "extends")]
pub mod extends;

#[cfg(feature = "soa")]
pub mod soa;
