/// Implements `TryFrom` between brand-typed scalar ids of the same brand, as
/// std's `TryFrom` between their primitives, failing with the same
/// `TryFromIntError`.
///
/// Each entry is `$src => $dst, ...;`. A pair whose primitives have `From`
/// fails to compile, since std's `TryFrom` for it cannot fail.
macro_rules! scalar_id_try_from {
    ( $( $src:ident => $( $dst:ident ),+ ; )+ ) => {
        $(
            $(
                impl<TBrand: ?Sized> ::std::convert::TryFrom<$src<TBrand>> for $dst<TBrand> {
                    type Error = ::std::num::TryFromIntError;

                    fn try_from(
                        id: $src<TBrand>,
                    ) -> ::std::result::Result<Self, ::std::num::TryFromIntError> {
                        let repr = $crate::internal::IdRepr::to_repr(id);

                        ::std::convert::TryFrom::try_from(repr)
                            .map($crate::internal::IdRepr::from_repr)
                    }
                }
            )+
        )+
    };
}

pub(crate) use scalar_id_try_from;
