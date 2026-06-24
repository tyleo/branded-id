use crate::string_ids::string_id_impl;
use std::ffi::{CStr, CString};

string_id_impl! {
    CStrId, CStr, from_c_str, as_c_str,
    CStringId, CString, from_c_string, into_c_string,
}

impl<TBrand: ?Sized> From<&CStr> for CStringId<TBrand> {
    fn from(value: &CStr) -> CStringId<TBrand> {
        CStringId::from_c_string(value.to_owned())
    }
}
