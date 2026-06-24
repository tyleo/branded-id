/// Builds a borrowed [`OsStrId`](crate::OsStrId) reference. Forms:
/// `os_str_id!(value)` (brand inferred) and `os_str_id!(Brand; value)`. Accepts
/// anything that is `AsRef<OsStr>`, such as a string literal or an `OsStr`.
///
/// # Examples
/// ```rust
/// use branded_id::{os_str_id, OsStrId};
/// use std::ffi::OsStr;
/// struct BFile;
/// let id: &OsStrId<BFile> = os_str_id!(BFile; "config.toml");
/// assert_eq!(id.as_os_str(), OsStr::new("config.toml"));
/// ```
#[macro_export]
macro_rules! os_str_id {
    ($id:expr) => {
        $crate::OsStrId::<_>::from_os_str(::std::convert::AsRef::<::std::ffi::OsStr>::as_ref(&$id))
    };
    ($brand:ty; $id:expr) => {
        $crate::OsStrId::<$brand>::from_os_str(::std::convert::AsRef::<::std::ffi::OsStr>::as_ref(
            &$id,
        ))
    };
}
