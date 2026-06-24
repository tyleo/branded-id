/// Builds a borrowed [`CStrId`](crate::CStrId) reference. Forms:
/// `c_str_id!(value)` (brand inferred) and `c_str_id!(Brand; value)`. Accepts
/// anything that is `AsRef<CStr>`, such as a `c"..."` literal or a `CString`.
///
/// # Examples
/// ```rust
/// use branded_id::{c_str_id, CStrId};
/// struct BSymbol;
/// let id: &CStrId<BSymbol> = c_str_id!(BSymbol; c"malloc");
/// assert_eq!(id.as_c_str(), c"malloc");
/// ```
#[macro_export]
macro_rules! c_str_id {
    ($id:expr) => {
        $crate::CStrId::<_>::from_c_str(::std::convert::AsRef::<::std::ffi::CStr>::as_ref(&$id))
    };
    ($brand:ty; $id:expr) => {
        $crate::CStrId::<$brand>::from_c_str(::std::convert::AsRef::<::std::ffi::CStr>::as_ref(
            &$id,
        ))
    };
}
