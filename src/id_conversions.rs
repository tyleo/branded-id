use crate::{
    I8Id, I16Id, I32Id, I64Id, I128Id, IsizeId, U8Id, U16Id, U32Id, U64Id, U128Id, UsizeId,
    internal,
};

// Generates every `to_*_id` cross-width conversion between the branded scalar
// ids from a single list. Adding a new id width means adding one line here.
internal::scalar_id_conversions! {
    (I8Id, i8, from_i8, to_i8, to_i8_id, "an"),
    (I16Id, i16, from_i16, to_i16, to_i16_id, "an"),
    (I32Id, i32, from_i32, to_i32, to_i32_id, "an"),
    (I64Id, i64, from_i64, to_i64, to_i64_id, "an"),
    (I128Id, i128, from_i128, to_i128, to_i128_id, "an"),
    (IsizeId, isize, from_isize, to_isize, to_isize_id, "an"),
    (U8Id, u8, from_u8, to_u8, to_u8_id, "a"),
    (U16Id, u16, from_u16, to_u16, to_u16_id, "a"),
    (U32Id, u32, from_u32, to_u32, to_u32_id, "a"),
    (U64Id, u64, from_u64, to_u64, to_u64_id, "a"),
    (U128Id, u128, from_u128, to_u128, to_u128_id, "a"),
    (UsizeId, usize, from_usize, to_usize, to_usize_id, "a"),
}
