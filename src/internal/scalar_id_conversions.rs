/// Generates the full matrix of cross-width `to_*_id` conversions between the
/// brand-typed scalar ids.
///
/// Given one list of every id type, this emits an `impl` block per type with a
/// `to_*_id` method for every *other* type, each performing an `as` cast. See
/// the type-level docs produced by [`scalar_id`](crate::internal::scalar_id)
/// for how those casts truncate or sign-reinterpret out-of-range values.
/// Keeping the whole matrix behind a single invocation means a new id width is
/// wired into every conversion by adding one line to the list.
///
/// Each entry is `($id, $prim, $from, $to, $to_id, $article)`: the id type, its
/// primitive, the constructor, the accessor, the name of the method that
/// converts *into* this type, and the indefinite article used for its doc.
macro_rules! scalar_id_conversions {
    ( $( $spec:tt ),+ $(,)? ) => {
        $crate::internal::scalar_id_conversions!(@outer [] [ $( $spec )+ ]);
    };

    // No more sources to expand.
    (@outer [ $( $before:tt )* ] []) => {};

    // Expand the next source against every other type (the sources already seen
    // plus the ones still ahead), then recurse with the source moved into the
    // seen list. Splitting the list around the source is what omits its own
    // identity conversion.
    (@outer [ $( $before:tt )* ] [ $src:tt $( $after:tt )* ]) => {
        $crate::internal::scalar_id_conversions!(@impl $src [ $( $before )* $( $after )* ]);
        $crate::internal::scalar_id_conversions!(@outer [ $( $before )* $src ] [ $( $after )* ]);
    };

    // Emit one source type's conversions to each listed target.
    (@impl
        ( $sid:ident, $sprim:ty, $sfrom:ident, $sto:ident, $sto_id:ident, $sarticle:literal )
        [ $( ( $tid:ident, $tprim:ty, $tfrom:ident, $tto:ident, $tto_id:ident, $tarticle:literal ) )* ]
    ) => {
        impl<TBrand: ?Sized> $sid<TBrand> {
            $(
                #[doc = ::std::concat!(
                    "Reinterprets the id as ", $tarticle, " [`", ::std::stringify!($tid),
                    "`] with an `as` cast."
                )]
                pub const fn $tto_id(self) -> $tid<TBrand> {
                    $tid::$tfrom(self.$sto() as $tprim)
                }
            )*
        }
    };
}

pub(crate) use scalar_id_conversions;
