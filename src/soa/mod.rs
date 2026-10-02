//! Columnar struct-of-arrays id pool. An [`IdStruct`] hands out and recycles
//! typed ids, and each [`IdField`] stores one column keyed by those ids. An
//! [`IdStructView`] borrows a pool beside some of its columns to read and
//! write rows without `unsafe`. An [`IdStructViewMut`] borrows every column
//! and can also add and remove rows. An [`IdList`] owns a pool and its one
//! column.

mod id_columns;
mod id_columns_mut;
mod id_columns_tuples;
mod id_field;
mod id_field_iter;
mod id_field_iter_mut;
mod id_list;
mod id_remap;
mod id_struct;
mod id_struct_iter;
mod id_struct_raw_parts;
mod id_struct_view;
mod id_struct_view_iter;
mod id_struct_view_iter_mut;
mod id_struct_view_mut;
mod u32_id_struct;

pub use id_columns::*;
pub use id_columns_mut::*;
pub use id_field::*;
pub use id_field_iter::*;
pub use id_field_iter_mut::*;
pub use id_list::*;
pub use id_remap::*;
pub use id_struct::*;
pub use id_struct_iter::*;
pub use id_struct_raw_parts::*;
pub use id_struct_view::*;
pub use id_struct_view_iter::*;
pub use id_struct_view_iter_mut::*;
pub use id_struct_view_mut::*;
pub use u32_id_struct::*;
