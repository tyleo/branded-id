/// Builds an owned [`OsStringId`](crate::OsStringId). Forms:
/// `os_string_id!(value)` (brand inferred) and `os_string_id!(Brand; value)`.
/// Accepts anything [`OsStringId`](crate::OsStringId) is `From`, such as a
/// string literal, a `String`, or an `OsString`.
///
/// # Examples
/// ```rust
/// use branded_id::{os_string_id, OsStringId};
/// use std::ffi::OsStr;
/// struct BFile;
/// let id: OsStringId<BFile> = os_string_id!(BFile; "config.toml");
/// assert_eq!(id.as_os_str(), OsStr::new("config.toml"));
/// ```
#[macro_export]
macro_rules! os_string_id {
    ($id:expr) => {
        $crate::OsStringId::<_>::from($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::OsStringId::<$brand>::from($id)
    };
}
