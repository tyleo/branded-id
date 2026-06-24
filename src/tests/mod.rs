pub mod util;

mod ext;
mod i128_id_tests;
mod i16_id_tests;
mod i32_id_tests;
mod i64_id_tests;
mod i8_id_tests;
mod id_array_tests;
mod id_ptr_tests;
mod id_slice_tests;
mod id_vec_tests;
mod internal;
mod isize_id_tests;
mod macros;
mod mut_id_ptr_tests;
mod string_ids;
mod u128_id_tests;
mod u16_id_tests;
mod u32_id_tests;
mod u64_id_tests;
mod u8_id_tests;
mod usize_id_tests;

#[cfg(feature = "extends")]
pub mod extends;

#[cfg(feature = "soa")]
pub mod soa;
