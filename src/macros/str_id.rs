/// Builds a borrowed [`StrId`](crate::StrId) reference. Forms: `str_id!(value)`
/// (brand inferred) and `str_id!(Brand; value)`. Accepts anything that is
/// `AsRef<str>`, such as a string literal or a `String`.
///
/// # Examples
/// ```rust
/// use branded_id::{str_id, StrId};
/// struct BUser;
/// let id: &StrId<BUser> = str_id!(BUser; "alice");
/// assert_eq!(id.as_str(), "alice");
/// ```
#[macro_export]
macro_rules! str_id {
    ($id:expr) => {
        $crate::StrId::<_>::from_str(::std::convert::AsRef::<str>::as_ref(&$id))
    };
    ($brand:ty; $id:expr) => {
        $crate::StrId::<$brand>::from_str(::std::convert::AsRef::<str>::as_ref(&$id))
    };
}
