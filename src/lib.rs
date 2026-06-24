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
//! struct BApples;
//! struct BOranges;
//!
//! let apple: UsizeId<BApples> = usize_id!(BApples; 2);
//! let orange: UsizeId<BOranges> = usize_id!(BOranges; 2);
//!
//! // Same underlying integer, but the brand keeps the domains apart.
//! assert_eq!(apple.to_usize(), orange.to_usize());
//! ```
//!
//! The crate provides a brand-typed integer id for each primitive integer
//! width (for example [`UsizeId`] and [`I32Id`]), the containers they index
//! ([`IdSlice`], [`IdArray`], [`IdVec`]) and pointers ([`IdPtr`],
//! [`MutIdPtr`]), plus the optional `extends` (brand conversions) and `soa`
//! (columnar id pools) modules. The `*_id!`, [`id_array!`], [`id_vec!`], and
//! [`id_slice!`] macros build them concisely.

#![warn(missing_docs)]

mod internal;
mod macros;

pub mod ext;

mod i128_id;
mod i16_id;
mod i32_id;
mod i64_id;
mod i8_id;
mod id;
mod id_array;
mod id_conversions;
mod id_ptr;
mod id_slice;
mod id_slice_index;
mod id_vec;
mod isize_id;
mod mut_id_ptr;
mod scalar;
mod u128_id;
mod u16_id;
mod u32_id;
mod u64_id;
mod u8_id;
mod usize_id;

pub use i8_id::*;
pub use i16_id::*;
pub use i32_id::*;
pub use i64_id::*;
pub use i128_id::*;
pub use id::*;
pub use id_array::*;
pub use id_ptr::*;
pub use id_slice::*;
pub use id_slice_index::*;
pub use id_vec::*;
pub use isize_id::*;
pub use mut_id_ptr::*;
pub use scalar::*;
pub use u8_id::*;
pub use u16_id::*;
pub use u32_id::*;
pub use u64_id::*;
pub use u128_id::*;
pub use usize_id::*;

#[cfg(feature = "extends")]
pub mod extends;

#[cfg(feature = "soa")]
pub mod soa;

#[cfg(test)]
mod tests;
