//! Data structures that are *branded* so they only interoperate with similarly
//! branded integer types.
//!
//! Every id and container in this crate carries a `TBrand` type parameter.
//! Two ids built for different brands are distinct types, so the compiler
//! rejects using one domain's id to index another domain's storage, even
//! though both are just integers at runtime.
//!
//! # Example
//! ```
//! use branded_id::{UsizeId, usize_id};
//!
//! struct Apples;
//! struct Oranges;
//!
//! let apple: UsizeId<Apples> = usize_id!(Apples; 2);
//! let orange: UsizeId<Oranges> = usize_id!(Oranges; 2);
//!
//! // Same underlying integer, but the brand keeps the domains apart.
//! assert_eq!(apple.to_usize(), orange.to_usize());
//! ```
//!
//! The crate provides brand-typed integer ids ([`UsizeId`], [`I32Id`],
//! [`U32Id`], [`IsizeId`]), the containers they index ([`IdSlice`],
//! [`IdArray`], [`IdVec`]) and pointers ([`IdPtr`], [`MutIdPtr`]), plus the
//! optional `extends` (brand conversions) and `soa` (columnar id pools)
//! modules. The `*_id!`, [`id_array!`], [`id_vec!`], and [`id_slice!`] macros
//! build them concisely.

mod internal;
mod macros;

pub mod ext;

mod i32_id;
mod id;
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
pub use id::*;
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
