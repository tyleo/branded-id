use crate::{IdSlice, internal::Sealed};

pub trait SliceExt<TValue>: Sealed {
    fn as_id_slice<TBrand: ?Sized>(&self) -> &IdSlice<TBrand, TValue>;

    fn as_mut_id_slice<TBrand: ?Sized>(&mut self) -> &mut IdSlice<TBrand, TValue>;
}

impl<TValue> SliceExt<TValue> for [TValue] {
    fn as_id_slice<TBrand: ?Sized>(&self) -> &IdSlice<TBrand, TValue> {
        IdSlice::from_slice(self)
    }

    fn as_mut_id_slice<TBrand: ?Sized>(&mut self) -> &mut IdSlice<TBrand, TValue> {
        IdSlice::from_mut_slice(self)
    }
}
