//! Columnar struct-of-arrays id pool. An [`IdStruct`] hands out and recycles
//! typed ids, and each [`IdField`] stores one column keyed by those ids.

mod id_field;
mod id_field_iter;
mod id_field_iter_mut;
mod id_remap;
mod id_struct;
mod id_struct_iter;
mod id_struct_raw_parts;
mod u32_id_struct;

pub use id_field::*;
pub use id_field_iter::*;
pub use id_field_iter_mut::*;
pub use id_remap::*;
pub use id_struct::*;
pub use id_struct_iter::*;
pub use id_struct_raw_parts::*;
pub use u32_id_struct::*;
