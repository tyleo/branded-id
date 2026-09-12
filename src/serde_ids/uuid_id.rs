use crate::UuidId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

impl<TBrand: ?Sized> Serialize for UuidId<TBrand> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_uuid().serialize(serializer)
    }
}

impl<'de, TBrand: ?Sized> Deserialize<'de> for UuidId<TBrand> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Uuid::deserialize(deserializer).map(Self::from_uuid)
    }
}
