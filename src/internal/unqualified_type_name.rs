use crate::internal::split_type_str;
use std::any::type_name;

pub fn unqualified_type_name<T: ?Sized>() -> String {
    split_type_str(type_name::<T>())
}
