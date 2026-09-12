/// Implements `Serialize` and `Deserialize` for a scalar id as its bare
/// integer.
macro_rules! scalar_id_serde {
    ($id:ident, $prim:ty, $from:ident, $to:ident) => {
        impl<TBrand: ?Sized> ::serde::Serialize for $crate::$id<TBrand> {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                ::serde::Serialize::serialize(&self.$to(), serializer)
            }
        }

        impl<'de, TBrand: ?Sized> ::serde::Deserialize<'de> for $crate::$id<TBrand> {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                <$prim as ::serde::Deserialize<'de>>::deserialize(deserializer).map(Self::$from)
            }
        }
    };
}

pub(crate) use scalar_id_serde;
