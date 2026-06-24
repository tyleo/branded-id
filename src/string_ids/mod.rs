//! Brand-typed string ids.
//!
//! Where the integer ids are indices, these are opaque branded keys: a
//! borrowed/owned pair for each standard string type. Each owned id derefs and
//! borrows to its borrowed half, so a map keyed by the owned id can be looked
//! up by a borrowed id without allocating.

mod c_str_id;
mod os_str_id;
mod path_id;
mod str_id;
mod string_id_impl;

pub(crate) use string_id_impl::string_id_impl;

pub use c_str_id::*;
pub use os_str_id::*;
pub use path_id::*;
pub use str_id::*;
