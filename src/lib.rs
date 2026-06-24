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
//!
//! Where the integer ids are indices, the crate also provides brand-typed
//! string ids that act as opaque keys: a borrowed/owned pair (like
//! [`StrId`]/[`StringId`]) for each standard string type. The owned id derefs
//! and borrows to its borrowed half, so a map keyed by the owned id can be
//! looked up by a borrowed id without allocating. Build them with the
//! [`str_id!`]/[`string_id!`] family of macros.

#![warn(missing_docs)]

mod internal;
mod macros;

pub mod ext;

mod containers;
mod scalar_ids;
mod string_ids;

pub use containers::*;
pub use scalar_ids::*;
pub use string_ids::*;

#[cfg(feature = "extends")]
pub mod extends;

#[cfg(feature = "soa")]
pub mod soa;

#[cfg(test)]
mod tests;
