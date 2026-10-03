use crate::string_ids::string_id_impl;
use std::ffi::{CStr, CString};

string_id_impl! {
    CStrId, CStr, from_c_str, as_c_str,
    CStringId, CString, from_c_string, into_c_string,
}
