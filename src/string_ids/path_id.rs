use crate::string_ids::string_id_impl;
use std::{
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
