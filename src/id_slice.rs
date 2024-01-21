use crate::{IdPtr, IdSliceIndex, MutIdPtr, UsizeId};
use std::{
    marker::PhantomData,
    mem::transmute,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct IdSlice<TMarker: ?Sized, TValue> {
    phantom: PhantomData<TMarker>,
    repr: [TValue],
}

impl<TMarker, TValue> IdSlice<TMarker, TValue> {
    pub fn from_mut_slice(repr: &mut [TValue]) -> &mut Self {
        unsafe { transmute(repr) }
    }

    pub const fn from_slice(repr: &[TValue]) -> &Self {
        unsafe { transmute(repr) }
    }

    pub fn as_mut_id_ptr(&mut self) -> MutIdPtr<TMarker, TValue> {
        MutIdPtr::from_mut_ptr(self.as_mut_slice().as_mut_ptr())
    }

    pub fn as_mut_slice(&mut self) -> &mut [TValue] {
        &mut self.repr
    }

    pub fn as_id_ptr(&self) -> IdPtr<TMarker, TValue> {
        IdPtr::from_ptr(self.as_slice().as_ptr())
    }

    pub const fn as_slice(&self) -> &[TValue] {
        &self.repr
    }

    pub const fn end(&self) -> UsizeId<TMarker> {
        UsizeId::from_usize(self.len())
    }

    pub const fn is_empty(&self) -> bool {
        self.repr.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.repr.len()
    }
}

impl<TMarker, TValue> AsMut<IdSlice<TMarker, TValue>> for IdSlice<TMarker, TValue> {
    fn as_mut(&mut self) -> &mut IdSlice<TMarker, TValue> {
        self
    }
}

impl<TMarker, TValue> AsRef<IdSlice<TMarker, TValue>> for IdSlice<TMarker, TValue> {
    fn as_ref(&self) -> &IdSlice<TMarker, TValue> {
        self
    }
}

impl<TMarker, TValue> Eq for IdSlice<TMarker, TValue> where [TValue]: PartialEq {}

impl<'a, TMarker, TValue> From<&'a [TValue]> for &'a IdSlice<TMarker, TValue> {
    fn from(value: &'a [TValue]) -> Self {
        IdSlice::from_slice(value)
    }
}

impl<'a, TMarker, TValue> From<&'a mut [TValue]> for &'a IdSlice<TMarker, TValue> {
    fn from(value: &'a mut [TValue]) -> Self {
        IdSlice::from_slice(value)
    }
}

impl<'a, TMarker, TValue> From<&'a mut [TValue]> for &'a mut IdSlice<TMarker, TValue> {
    fn from(value: &'a mut [TValue]) -> Self {
        IdSlice::from_mut_slice(value)
    }
}

impl<TMarker, TValue, I: IdSliceIndex<IdSlice<TMarker, TValue>>> Index<I>
    for IdSlice<TMarker, TValue>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        index.index(self)
    }
}

impl<TMarker, TValue, I: IdSliceIndex<IdSlice<TMarker, TValue>>> IndexMut<I>
    for IdSlice<TMarker, TValue>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_mut(self)
    }
}

impl<TMarker, TValueA, TValueB> PartialEq<IdSlice<TMarker, TValueB>> for IdSlice<TMarker, TValueA>
where
    [TValueA]: PartialEq<[TValueB]>,
{
    fn eq(&self, other: &IdSlice<TMarker, TValueB>) -> bool {
        self.repr.eq(&other.repr)
    }
}
