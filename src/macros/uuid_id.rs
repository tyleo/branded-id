/// Builds a [`UuidId`](crate::UuidId) from a [`Uuid`](uuid::Uuid). Forms:
/// `uuid_id!(value)` (brand inferred) and `uuid_id!(Brand; value)`.
///
/// # Examples
/// ```rust
/// use branded_id::{uuid_id, UuidId};
/// use uuid::Uuid;
/// struct BUser;
/// let raw = Uuid::from_u128(0x1234);
/// let id: UuidId<BUser> = uuid_id!(BUser; raw);
/// assert_eq!(id.to_uuid(), raw);
/// ```
#[macro_export]
macro_rules! uuid_id {
    ($id:expr) => {
        $crate::UuidId::<_>::from_uuid($id)
    };
    ($brand:ty; $id:expr) => {
        $crate::UuidId::<$brand>::from_uuid($id)
    };
}
