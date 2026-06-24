use crate::{U128Id, UuidId};
use uuid::Uuid;

impl<TBrand: ?Sized> U128Id<TBrand> {
    /// Reinterprets the id as a [`UuidId`] of the same brand, writing the
    /// integer as the UUID's 16 bytes in big-endian order. Lossless and the
    /// inverse of [`UuidId::to_u128_id`].
    pub const fn to_uuid_id(self) -> UuidId<TBrand> {
        UuidId::from_uuid(Uuid::from_u128(self.to_u128()))
    }
}
