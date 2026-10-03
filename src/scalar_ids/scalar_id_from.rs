/// Implements `From` between brand-typed scalar ids of the same brand, as
/// std's `From` between their primitives.
///
/// Each entry is `$src => $dst, ...;`. A pair whose primitives lack `From`
/// fails to compile.
macro_rules! scalar_id_from {
    ( $( $src:ident => $( $dst:ident ),+ ; )+ ) => {
        $(
            $(
                impl<TBrand: ?Sized> ::std::convert::From<$src<TBrand>> for $dst<TBrand> {
                    fn from(id: $src<TBrand>) -> Self {
                        let repr = $crate::internal::IdRepr::to_repr(id);

                        $crate::internal::IdRepr::from_repr(::std::convert::From::from(repr))
                    }
                }
            )+
        )+
    };
}

pub(crate) use scalar_id_from;
