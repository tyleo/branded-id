/// Builds a borrowed [`PathId`](crate::PathId) reference. Forms:
/// `path_id!(value)` (brand inferred) and `path_id!(Brand; value)`. Accepts
/// anything that is `AsRef<Path>`, such as a string literal or a `Path`.
///
/// # Examples
/// ```rust
/// use branded_id::{path_id, PathId};
/// use std::path::Path;
/// struct BAsset;
/// let id: &PathId<BAsset> = path_id!(BAsset; "tex/wall.png");
/// assert_eq!(id.as_path(), Path::new("tex/wall.png"));
/// ```
#[macro_export]
macro_rules! path_id {
    ($id:expr) => {
        $crate::PathId::<_>::from_path(::std::convert::AsRef::<::std::path::Path>::as_ref(&$id))
    };
    ($brand:ty; $id:expr) => {
        $crate::PathId::<$brand>::from_path(::std::convert::AsRef::<::std::path::Path>::as_ref(
            &$id,
        ))
    };
}
