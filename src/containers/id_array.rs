use crate::{IdSlice, IdSliceIndex};
use std::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

/// A `[TValue; N]` array indexed by brand-typed ids instead of bare `usize`.
#[repr(transparent)]
pub struct IdArray<TBrand: ?Sized, TValue, const N: usize> {
    phantom: PhantomData<TBrand>,
    repr: [TValue; N],
}

impl<TBrand: ?Sized, TValue, const N: usize> IdArray<TBrand, TValue, N> {
    /// Borrows the underlying `[TValue; N]`.
    pub const fn as_array(&self) -> &[TValue; N] {
        &self.repr
    }

    /// Borrows the underlying `[TValue; N]` mutably.
    pub fn as_mut_array(&mut self) -> &mut [TValue; N] {
        &mut self.repr
    }

    /// Borrows the elements as a mutable [`IdSlice`].
    pub fn as_mut_id_slice(&mut self) -> &mut IdSlice<TBrand, TValue> {
        IdSlice::from_mut_slice(self.as_mut_array())
    }

    /// Borrows the elements as an [`IdSlice`].
    pub const fn as_id_slice(&self) -> &IdSlice<TBrand, TValue> {
        IdSlice::from_slice(self.as_array())
    }

    /// Wraps an owned `[TValue; N]` as an [`IdArray`].
    pub const fn from_array(repr: [TValue; N]) -> Self {
        Self {
            phantom: PhantomData,
            repr,
        }
    }

    /// Reinterprets a `[TValue; N]` as an [`IdArray`].
    pub const fn from_array_ref(repr: &[TValue; N]) -> &Self {
        // SAFETY: IdArray is #[repr(transparent)] over [TValue; N], so &[TValue; N]
        // and &IdArray share a layout.
        unsafe { transmute(repr) }
    }

    /// Reinterprets a mutable `[TValue; N]` as a mutable [`IdArray`].
    pub fn from_mut_array(repr: &mut [TValue; N]) -> &mut Self {
        // SAFETY: IdArray is #[repr(transparent)] over [TValue; N], so &mut [TValue; N]
        // and &mut IdArray share a layout.
        unsafe { transmute(repr) }
    }

    /// Unwraps into the owned `[TValue; N]`.
    #[must_use]
    pub fn into_array(self) -> [TValue; N] {
        self.repr
    }

    /// Returns an iterator over shared references to the elements.
    pub fn iter(&self) -> Iter<'_, TValue> {
        self.as_array().iter()
    }

    /// Returns an iterator over mutable references to the elements.
    pub fn iter_mut(&mut self) -> IterMut<'_, TValue> {
        self.as_mut_array().iter_mut()
    }
}

impl<TBrand: ?Sized, TValue, const N: usize> AsMut<IdSlice<TBrand, TValue>>
    for IdArray<TBrand, TValue, N>
{
    fn as_mut(&mut self) -> &mut IdSlice<TBrand, TValue> {
        self.as_mut_id_slice()
    }
}

impl<TBrand: ?Sized, TValue, const N: usize> AsRef<IdSlice<TBrand, TValue>>
    for IdArray<TBrand, TValue, N>
{
    fn as_ref(&self) -> &IdSlice<TBrand, TValue> {
        self.as_id_slice()
    }
}

impl<TBrand: ?Sized, TValue, const N: usize> Borrow<IdSlice<TBrand, TValue>>
    for IdArray<TBrand, TValue, N>
{
    fn borrow(&self) -> &IdSlice<TBrand, TValue> {
        IdSlice::from_slice(self.as_array().borrow())
    }
}

impl<TBrand: ?Sized, TValue, const N: usize> BorrowMut<IdSlice<TBrand, TValue>>
    for IdArray<TBrand, TValue, N>
{
    fn borrow_mut(&mut self) -> &mut IdSlice<TBrand, TValue> {
        IdSlice::from_mut_slice(self.as_mut_array().borrow_mut())
    }
}

impl<TBrand: ?Sized, TValue, const N: usize> Clone for IdArray<TBrand, TValue, N>
where
    [TValue; N]: Clone,
{
    fn clone(&self) -> Self {
        Self::from_array(self.as_array().clone())
    }
}

impl<TBrand: ?Sized, TValue, const N: usize> Copy for IdArray<TBrand, TValue, N> where
    [TValue; N]: Copy
{
}

impl<TBrand: ?Sized, TValue: Debug, const N: usize> Debug for IdArray<TBrand, TValue, N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self.as_id_slice(), f)
    }
}

impl<TBrand: ?Sized, TValue, const N: usize> Default for IdArray<TBrand, TValue, N>
where
    [TValue; N]: Default,
{
    fn default() -> Self {
        Self::from_array(Default::default())
    }
}

impl<TBrand: ?Sized, TValue, const N: usize> Eq for IdArray<TBrand, TValue, N> where
    [TValue; N]: PartialEq
{
}

impl<TBrand: ?Sized, TValue, const N: usize> From<[TValue; N]> for IdArray<TBrand, TValue, N> {
    fn from(value: [TValue; N]) -> Self {
        Self::from_array(value)
    }
}

impl<TBrand: ?Sized, TValue, const N: usize> Hash for IdArray<TBrand, TValue, N>
where
    [TValue; N]: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_array().hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        // SAFETY: IdArray is #[repr(transparent)] over [TValue; N], so &[IdArray]
        // and &[[TValue; N]] share a layout.
        let data = unsafe { transmute::<&[IdArray<TBrand, TValue, N>], &[[TValue; N]]>(data) };
        <[TValue; N]>::hash_slice(data, state)
    }
}

impl<TBrand: ?Sized, TValue, I: IdSliceIndex<IdSlice<TBrand, TValue>>, const N: usize> Index<I>
    for IdArray<TBrand, TValue, N>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        index.index(self.as_id_slice())
    }
}

impl<TBrand: ?Sized, TValue, I: IdSliceIndex<IdSlice<TBrand, TValue>>, const N: usize> IndexMut<I>
    for IdArray<TBrand, TValue, N>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_mut(self.as_mut_id_slice())
    }
}

#[allow(clippy::into_iter_on_ref)]
impl<'a, TBrand: ?Sized, TValue, const N: usize> IntoIterator for &'a IdArray<TBrand, TValue, N> {
    type Item = <&'a [TValue; N] as IntoIterator>::Item;
    type IntoIter = <&'a [TValue; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.as_array().into_iter()
    }
}

#[allow(clippy::into_iter_on_ref)]
impl<'a, TBrand: ?Sized, TValue, const N: usize> IntoIterator
    for &'a mut IdArray<TBrand, TValue, N>
{
    type Item = <&'a mut [TValue; N] as IntoIterator>::Item;
    type IntoIter = <&'a mut [TValue; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_array().into_iter()
    }
}

impl<TBrand: ?Sized, TValue, const N: usize> IntoIterator for IdArray<TBrand, TValue, N> {
    type Item = <[TValue; N] as IntoIterator>::Item;
    type IntoIter = <[TValue; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}

impl<TBrand: ?Sized, TValue: Ord, const N: usize> Ord for IdArray<TBrand, TValue, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_array().cmp(other.as_array())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_array(self.into_array().max(other.into_array()))
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_array(self.into_array().min(other.into_array()))
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        Self::from_array(self.into_array().clamp(min.into_array(), max.into_array()))
    }
}

impl<'a, TBrand: ?Sized, TValueA, TValueB, const N: usize> PartialEq<&'a IdSlice<TBrand, TValueB>>
    for IdArray<TBrand, TValueA, N>
where
    [TValueA; N]: PartialEq<&'a [TValueB]>,
{
    fn eq(&self, other: &&'a IdSlice<TBrand, TValueB>) -> bool {
        self.as_array().eq(&other.as_slice())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &&'a IdSlice<TBrand, TValueB>) -> bool {
        self.as_array().ne(&other.as_slice())
    }
}

impl<'a, TBrand: ?Sized, TValueA, TValueB, const N: usize>
    PartialEq<&'a mut IdSlice<TBrand, TValueB>> for IdArray<TBrand, TValueA, N>
where
    [TValueA; N]: PartialEq<&'a mut [TValueB]>,
{
    fn eq(&self, other: &&'a mut IdSlice<TBrand, TValueB>) -> bool {
        // SAFETY: IdSlice is #[repr(transparent)] over [TValueB], so the
        // reference reinterprets to a &&mut [TValueB] of identical layout.
        let other: &&'a mut _ = unsafe { transmute(other) };
        self.as_array().eq(other)
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &&'a mut IdSlice<TBrand, TValueB>) -> bool {
        // SAFETY: IdSlice is #[repr(transparent)] over [TValueB], so the
        // reference reinterprets to a &&mut [TValueB] of identical layout.
        let other: &&'a mut _ = unsafe { transmute(other) };
        self.as_array().ne(other)
    }
}

impl<TBrand: ?Sized, TValueA, TValueB, const N: usize> PartialEq<IdSlice<TBrand, TValueB>>
    for IdArray<TBrand, TValueA, N>
where
    [TValueA; N]: PartialEq<[TValueB]>,
{
    fn eq(&self, other: &IdSlice<TBrand, TValueB>) -> bool {
        self.as_array().eq(other.as_slice())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdSlice<TBrand, TValueB>) -> bool {
        self.as_array().ne(other.as_slice())
    }
}

impl<TBrand: ?Sized, TValueA, TValueB, const N: usize> PartialEq<IdArray<TBrand, TValueB, N>>
    for IdArray<TBrand, TValueA, N>
where
    [TValueA; N]: PartialEq<[TValueB; N]>,
{
    fn eq(&self, other: &IdArray<TBrand, TValueB, N>) -> bool {
        self.as_array().eq(other.as_array())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdArray<TBrand, TValueB, N>) -> bool {
        self.as_array().ne(other.as_array())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TBrand: ?Sized, TValue: PartialOrd, const N: usize> PartialOrd for IdArray<TBrand, TValue, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_array().partial_cmp(other.as_array())
    }

    fn lt(&self, other: &Self) -> bool {
        self.as_array().lt(other.as_array())
    }

    fn le(&self, other: &Self) -> bool {
        self.as_array().le(other.as_array())
    }

    fn gt(&self, other: &Self) -> bool {
        self.as_array().gt(other.as_array())
    }

    fn ge(&self, other: &Self) -> bool {
        self.as_array().ge(other.as_array())
    }
}
