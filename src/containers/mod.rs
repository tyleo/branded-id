//! Containers and pointers indexed by brand-typed ids.
//!
//! [`IdSlice`], [`IdArray`], and [`IdVec`] mirror `[T]`, `[T; N]`, and
//! `Vec<T>`, but are indexed by [`UsizeId`](crate::UsizeId) and id ranges
//! instead of bare `usize`, and [`IdPtr`]/[`MutIdPtr`] are the matching
//! brand-typed raw pointers. [`IdSliceIndex`] is the indexing trait that ties
//! an id or id range to the element or sub-slice it selects.

mod id_array;
mod id_ptr;
mod id_slice;
mod id_slice_index;
mod id_vec;
mod mut_id_ptr;

pub use id_array::*;
pub use id_ptr::*;
pub use id_slice::*;
pub use id_slice_index::*;
pub use id_vec::*;
pub use mut_id_ptr::*;
