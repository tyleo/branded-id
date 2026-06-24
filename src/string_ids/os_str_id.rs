use crate::string_ids::string_id_impl;
use std::ffi::{OsStr, OsString};

string_id_impl! {
    OsStrId, OsStr, from_os_str, as_os_str,
    OsStringId, OsString, from_os_string, into_os_string,
}

impl<TBrand: ?Sized> From<&OsStr> for OsStringId<TBrand> {
    fn from(value: &OsStr) -> OsStringId<TBrand> {
        OsStringId::from_os_string(value.to_owned())
    }
}

impl<TBrand: ?Sized> From<&str> for OsStringId<TBrand> {
    fn from(value: &str) -> OsStringId<TBrand> {
        OsStringId::from_os_string(OsString::from(value))
    }
}

impl<TBrand: ?Sized> From<String> for OsStringId<TBrand> {
    fn from(value: String) -> OsStringId<TBrand> {
        OsStringId::from_os_string(OsString::from(value))
    }
}
