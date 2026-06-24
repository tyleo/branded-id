use crate::string_ids::string_id_impl;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    str::FromStr,
};

string_id_impl! {
    PathId, Path, from_path, as_path,
    PathBufId, PathBuf, from_path_buf, into_path_buf,
}

impl<TBrand: ?Sized> FromStr for PathBufId<TBrand> {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PathBufId::from_path_buf(<PathBuf as FromStr>::from_str(s)?))
    }
}

impl<TBrand: ?Sized> From<&Path> for PathBufId<TBrand> {
    fn from(value: &Path) -> PathBufId<TBrand> {
        PathBufId::from_path_buf(value.to_owned())
    }
}

impl<TBrand: ?Sized> From<&str> for PathBufId<TBrand> {
    fn from(value: &str) -> PathBufId<TBrand> {
        PathBufId::from_path_buf(PathBuf::from(value))
    }
}

impl<TBrand: ?Sized> From<String> for PathBufId<TBrand> {
    fn from(value: String) -> PathBufId<TBrand> {
        PathBufId::from_path_buf(PathBuf::from(value))
    }
}

impl<TBrand: ?Sized> From<OsString> for PathBufId<TBrand> {
    fn from(value: OsString) -> PathBufId<TBrand> {
        PathBufId::from_path_buf(PathBuf::from(value))
    }
}
