use crate::{IdSlice, internal::Sealed};

pub trait SliceExt<TValue>: Sealed {
    fn as_id_slice<TMarker: ?Sized>(&self) -> &IdSlice<TMarker, TValue>;

    fn as_mut_id_slice<TMarker: ?Sized>(&mut self) -> &mut IdSlice<TMarker, TValue>;
}

impl<TValue> SliceExt<TValue> for [TValue] {
    fn as_id_slice<TMarker: ?Sized>(&self) -> &IdSlice<TMarker, TValue> {
        IdSlice::from_slice(self)
    }

    fn as_mut_id_slice<TMarker: ?Sized>(&mut self) -> &mut IdSlice<TMarker, TValue> {
        IdSlice::from_mut_slice(self)
    }
}
