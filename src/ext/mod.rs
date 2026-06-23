//! Extension traits that add id-typed views to primitives, slices, arrays,
//! `Vec`, and raw pointers (for example `as_id_slice` and `to_i32_id`). The
//! traits are sealed: this crate provides every implementation.

mod array_ext;
mod bound_pair_ext;
mod i32_ext;
mod isize_ext;
mod mut_ptr_ext;
mod ptr_ext;
mod range_ext;
mod range_from_ext;
mod range_full_ext;
mod range_inclusive_ext;
mod range_to_ext;
mod range_to_inclusive_ext;
mod slice_ext;
mod u32_ext;
mod usize_ext;
mod vec_ext;

pub use array_ext::*;
pub use i32_ext::*;
pub use isize_ext::*;
pub use mut_ptr_ext::*;
pub use ptr_ext::*;
pub use slice_ext::*;
pub use u32_ext::*;
pub use usize_ext::*;
pub use vec_ext::*;
