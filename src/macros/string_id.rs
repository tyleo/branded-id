/// Builds an owned [`StringId`](crate::StringId). Forms: `string_id!(value)`
/// (brand inferred) and `string_id!(Brand; value)`. Accepts anything
/// `String::from` takes, such as a string literal or a `String`.
///
/// # Examples
/// ```rust
/// use branded_id::{string_id, StringId};
/// struct BUser;
/// let id: StringId<BUser> = string_id!(BUser; "alice");
/// assert_eq!(id.as_str(), "alice");
/// ```
#[macro_export]
macro_rules! string_id {
    ($id:expr) => {
        $crate::StringId::<_>::from_string(::std::string::String::from($id))
    };
    ($brand:ty; $id:expr) => {
        $crate::StringId::<$brand>::from_string(::std::string::String::from($id))
    };
}
