use crate::string_ids::string_id_impl;
use std::ffi::{OsStr, OsString};

string_id_impl! {
    OsStrId, OsStr, from_os_str, as_os_str,
    OsStringId, OsString, from_os_string, into_os_string,
}
