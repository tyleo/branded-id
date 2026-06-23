/// Generates the `IdSliceIndex` impl for a `std::ops` range type keyed by
/// `UsizeId`, reinterpreting it as the matching primitive range before
/// indexing the backing slice.
///
/// `$range` is the range type (e.g. `Range`) and `$converter` the `UsizeId`
/// associated fn that reinterprets `$range<UsizeId<_>>` as `$range<usize>`.
macro_rules! id_slice_range_index {
    ($range:ident, $converter:ident) => {
        impl<TMarker: ?Sized, TValue> $crate::IdSliceIndex<$crate::IdSlice<TMarker, TValue>>
            for ::std::ops::$range<$crate::UsizeId<TMarker>>
        {
            type Output = $crate::IdSlice<TMarker, TValue>;

            fn get(self, slice: &$crate::IdSlice<TMarker, TValue>) -> Option<&Self::Output> {
                let range = $crate::UsizeId::$converter(self);
                let slice = slice.as_slice().get(range)?;
                Some($crate::IdSlice::from_slice(slice))
            }

            fn get_mut(
                self,
                slice: &mut $crate::IdSlice<TMarker, TValue>,
            ) -> Option<&mut Self::Output> {
                let range = $crate::UsizeId::$converter(self);
                let slice = slice.as_mut_slice().get_mut(range)?;
                Some($crate::IdSlice::from_mut_slice(slice))
            }

            fn index(self, slice: &$crate::IdSlice<TMarker, TValue>) -> &Self::Output {
                let range = $crate::UsizeId::$converter(self);
                $crate::IdSlice::from_slice(&slice.as_slice()[range])
            }

            fn index_mut(self, slice: &mut $crate::IdSlice<TMarker, TValue>) -> &mut Self::Output {
                let range = $crate::UsizeId::$converter(self);
                $crate::IdSlice::from_mut_slice(&mut slice.as_mut_slice()[range])
            }
        }
    };
}

pub(crate) use id_slice_range_index;
