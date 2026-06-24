/// Builds an owned [`CStringId`](crate::CStringId). Forms:
/// `c_string_id!(value)` (brand inferred) and `c_string_id!(Brand; value)`.
/// Accepts anything [`CStringId`](crate::CStringId) is `From`, such as a
/// `c"..."` literal or a `CString`.
///
/// Building a `CString` from arbitrary bytes is fallible (interior nul bytes),
/// so unlike the other owned string ids this does not accept a plain `&str`;
/// pass a `c"..."` literal or an already-built `CString`.
///
/// # Examples
/// ```rust
/// use branded_id::{c_string_id, CStringId};
/// struct BSymbol;
/// let id: CStringId<BSymbol> = c_string_id!(BSymbol; c"malloc");
/// assert_eq!(id.as_c_str(), c"malloc");
/// ```
#[macro_export]
macro_rules! c_string_id {
    ($id:expr) => {
        $crate::CStringId::<_>::from($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::CStringId::<$brand>::from($id)
    };
}
