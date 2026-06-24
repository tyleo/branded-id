//! Data structures that are *branded* so they only interoperate with similarly
//! branded integer types.
//!
//! Every id and container in this crate carries a `TBrand` type parameter.
//! Two ids built for different brands are distinct types, so the compiler
//! rejects using one domain's id to index another domain's storage, even
//! though both are just integers at runtime.
//!
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
//! # Integer Ids
//! A brand-typed integer id for each primitive width (for example [`UsizeId`]
//! and [`I32Id`]), built with the `*_id!` macros. [`UsizeId`] is the canonical
//! width that indexes storage; the others convert through it.
//!
//! # Containers
//! Brand-typed slices, arrays, vectors, and pointers that only accept ids of
//! their own brand. Build them with [`id_array!`], [`id_vec!`], and
//! [`id_slice!`].
//!
//! # String Ids
//! Opaque brand-typed keys rather than indices: a borrowed/owned pair (like
//! [`StrId`]/[`StringId`]) for each standard string type, built with
//! [`str_id!`]/[`string_id!`]. The owned id derefs to its borrowed half, so a
//! map keyed by the owned id can be looked up by a borrowed id without
//! allocating.
//!
//! # Extension Traits (`ext`)
//! Sealed extension traits that add id-typed views to primitives, slices,
//! arrays, `Vec`, and raw pointers.
//!
//! # Brand Conversions (`extends`, default feature)
//! When one brand extends another, ids and containers cast between the two
//! with `upcast`/`downcast` methods.
//!
//! # Struct of Arrays (`soa`, default feature)
//! A columnar struct-of-arrays id pool. An `IdStruct` hands out and recycles
//! typed ids, and each `IdField` stores one column keyed by those ids, so an
//! entity's data lives across parallel columns rather than in one struct.
//!
//! ```
//! # #[cfg(feature = "soa")]
//! # fn main() {
//! use branded_id::soa::{IdField, IdStruct};
//!
//! struct BEnemy;
//!
//! let mut enemies = IdStruct::<BEnemy>::new();
//! let mut health = IdField::<BEnemy, i32>::new();
//! let mut attack = IdField::<BEnemy, i32>::new();
//!
//! // Spawn an enemy by retaining an id, then give it a value in each column.
//! let goblin = enemies.retain();
//! health.retain(goblin, 30);
//! attack.retain(goblin, 5);
//!
//! let troll = enemies.retain();
//! health.retain(troll, 80);
//! attack.retain(troll, 12);
//!
//! // The pool tracks liveness, so reading a column is unsafe: the caller
//! // vouches that the id is retained and has a value here.
//! // SAFETY: goblin and troll are retained with a value in each field.
//! unsafe {
//!     // The troll hits the goblin.
//!     *health.get_mut(goblin) -= *attack.get(troll);
//!     assert_eq!(*health.get(goblin), 18);
//! }
//! # }
//! # #[cfg(not(feature = "soa"))]
//! # fn main() {}
//! ```
//!
//! # UUID Ids (`uuid`, optional feature)
//! Adds `UuidId`, an opaque brand-typed `Uuid` key with `from_uuid`/`to_uuid`
//! conversions and a `uuid_id!` macro.

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

#[cfg(feature = "uuid")]
mod uuid_ids;

#[cfg(feature = "uuid")]
pub use uuid_ids::*;

#[cfg(test)]
mod tests;
