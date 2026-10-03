//! Extension traits that add id-typed views to slices, arrays, `Vec`, and raw
//! pointers (for example `as_id_slice`), and id adapters: `enumerate_ids` for
//! iterators and `into_id_range` for ranges of ids. The traits are sealed or
//! implemented for every iterator, so this crate provides every implementation.

mod array_ext;
mod bound_pair_ext;
mod iterator_ext;
mod mut_ptr_ext;
mod ptr_ext;
mod range_ext;
mod range_from_ext;
mod range_full_ext;
mod range_inclusive_ext;
mod range_to_ext;
mod range_to_inclusive_ext;
mod slice_ext;
mod vec_ext;

pub use array_ext::*;
pub use iterator_ext::*;
pub use mut_ptr_ext::*;
pub use ptr_ext::*;
pub use range_ext::*;
pub use range_inclusive_ext::*;
pub use slice_ext::*;
pub use vec_ext::*;
