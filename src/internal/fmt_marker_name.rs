use crate::internal::unqualified_type_name;
use std::{any::type_name, fmt, fmt::Formatter};

pub fn fmt_marker_name<TMarker>(f: &mut Formatter) -> fmt::Result {
    if f.alternate() {
        let type_name = type_name::<TMarker>();
        f.write_str(type_name)
    } else {
        let type_name = unqualified_type_name::<TMarker>();
        f.write_str(&type_name)
    }
}
