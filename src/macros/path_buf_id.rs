/// Builds an owned [`PathBufId`](crate::PathBufId). Forms:
/// `path_buf_id!(value)` (brand inferred) and `path_buf_id!(Brand; value)`.
/// Accepts anything `PathBuf::from` takes, such as a string literal, a
/// `String`, a `PathBuf`, or an `OsString`.
///
/// # Examples
/// ```rust
/// use branded_id::{path_buf_id, PathBufId};
/// use std::path::Path;
/// struct BAsset;
/// let id: PathBufId<BAsset> = path_buf_id!(BAsset; "tex/wall.png");
/// assert_eq!(id.as_path(), Path::new("tex/wall.png"));
/// ```
#[macro_export]
macro_rules! path_buf_id {
    ($id:expr) => {
        $crate::PathBufId::<_>::from_path_buf(::std::path::PathBuf::from($id))
    };
    ($brand:ty; $id:expr) => {
        $crate::PathBufId::<$brand>::from_path_buf(::std::path::PathBuf::from($id))
    };
}
