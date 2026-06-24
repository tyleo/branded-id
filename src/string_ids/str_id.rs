use crate::string_ids::string_id_impl;
use std::{fmt, str::FromStr};

string_id_impl! {
    StrId, str, from_str, as_str,
    StringId, String, from_string, into_string,
}

impl<TBrand: ?Sized> fmt::Display for StrId<TBrand> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_helper(fmt::Display::fmt, f)
    }
}

impl<TBrand: ?Sized> fmt::Display for StringId<TBrand> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_helper(fmt::Display::fmt, f)
    }
}

impl<TBrand: ?Sized> FromStr for StringId<TBrand> {
    type Err = <String as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringId::from_string(<String as FromStr>::from_str(s)?))
    }
}

impl<TBrand: ?Sized> From<&str> for StringId<TBrand> {
    fn from(value: &str) -> StringId<TBrand> {
        StringId::from_string(value.to_owned())
    }
}
