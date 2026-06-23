use crate::internal::unqualified_type_name;
use std::{any::type_name, fmt, fmt::Formatter};

pub fn fmt_brand_name<TBrand: ?Sized>(f: &mut Formatter) -> fmt::Result {
    if f.alternate() {
        let type_name = type_name::<TBrand>();
        f.write_str(type_name)
    } else {
        let type_name = unqualified_type_name::<TBrand>();
        f.write_str(&type_name)
    }
}
