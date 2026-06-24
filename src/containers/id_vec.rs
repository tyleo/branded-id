use crate::{IdArray, IdSlice, IdSliceIndex, UsizeId};
use std::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::transmute,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// A `Vec<TValue>` indexed by brand-typed ids instead of bare `usize`.
#[repr(transparent)]
pub struct IdVec<TBrand: ?Sized, TValue> {
    phantom: PhantomData<TBrand>,
    repr: Vec<TValue>,
}

impl<TBrand: ?Sized, TValue> IdVec<TBrand, TValue> {
    /// Borrows the contents as a mutable [`IdSlice`].
    pub fn as_mut_id_slice(&mut self) -> &mut IdSlice<TBrand, TValue> {
        IdSlice::from_mut_slice(&mut self.repr)
    }

    /// Borrows the underlying `Vec` mutably.
    pub fn as_mut_vec(&mut self) -> &mut Vec<TValue> {
        &mut self.repr
    }

    /// Borrows the contents as an [`IdSlice`].
    pub fn as_id_slice(&self) -> &IdSlice<TBrand, TValue> {
        IdSlice::from_slice(&self.repr)
    }

    /// Borrows the underlying `Vec`.
    pub const fn as_vec(&self) -> &Vec<TValue> {
        &self.repr
    }

    /// Returns the number of elements the vec can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.repr.capacity()
    }

    /// Removes all elements, keeping the allocated capacity.
    pub fn clear(&mut self) {
        self.repr.clear()
    }

    /// Returns the [`UsizeId`] one past the last element, equal to the length.
    pub fn end(&self) -> UsizeId<TBrand> {
        self.as_id_slice().end()
    }

    /// Appends a clone of each element of `other`.
    pub fn extend_from_slice(&mut self, other: &[TValue])
    where
        TValue: Clone,
    {
        self.repr.extend_from_slice(other)
    }

    /// Reinterprets a mutable `Vec` reference as a mutable [`IdVec`].
    pub fn from_mut_vec(vec: &mut Vec<TValue>) -> &mut Self {
        // SAFETY: IdVec is #[repr(transparent)] over Vec<TValue>, so &mut Vec
        // and &mut IdVec share a layout.
        unsafe { transmute(vec) }
    }

    /// Wraps an owned `Vec`.
    pub const fn from_vec(vec: Vec<TValue>) -> Self {
        Self {
            phantom: PhantomData,
            repr: vec,
        }
    }

    /// Reinterprets a `Vec` reference as an [`IdVec`].
    pub const fn from_vec_ref(vec: &Vec<TValue>) -> &Self {
        // SAFETY: IdVec is #[repr(transparent)] over Vec<TValue>, so &Vec and
        // &IdVec share a layout.
        unsafe { transmute(vec) }
    }

    /// Inserts `value` at `index`, shifting the later elements up.
    ///
    /// # Panics
    /// Panics if `index` is greater than the length.
    pub fn insert(&mut self, index: UsizeId<TBrand>, value: TValue) {
        self.repr.insert(index.to_usize(), value)
    }

    /// Unwraps into the owned `Vec`.
    #[must_use]
    pub fn into_vec(self) -> Vec<TValue> {
        self.repr
    }

    /// Returns whether there are no elements.
    pub fn is_empty(&self) -> bool {
        self.repr.is_empty()
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.repr.len()
    }

    /// Creates an empty vec that reserves no storage up front.
    pub const fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    /// Removes and returns the last element, or `None` if empty.
    pub fn pop(&mut self) -> Option<TValue> {
        self.repr.pop()
    }

    /// Appends `value` and returns the [`UsizeId`] of the newly appended element.
    pub fn push(&mut self, value: TValue) -> UsizeId<TBrand> {
        let res = self.end();
        self.repr.push(value);
        res
    }

    /// Removes and returns the element at `index`, shifting the later elements down.
    ///
    /// # Panics
    /// Panics if `index` is out of bounds.
    pub fn remove(&mut self, index: UsizeId<TBrand>) -> TValue {
        self.repr.remove(index.to_usize())
    }

    /// Reserves capacity for at least `additional` more elements.
    pub fn reserve(&mut self, additional: usize) {
        self.repr.reserve(additional)
    }

    /// Reserves capacity for exactly `additional` more elements.
    pub fn reserve_exact(&mut self, additional: usize) {
        self.repr.reserve_exact(additional)
    }

    /// Resizes to `new_len`, cloning `value` to fill any new slots.
    pub fn resize(&mut self, new_len: usize, value: TValue)
    where
        TValue: Clone,
    {
        self.repr.resize(new_len, value)
    }

    /// Drops excess capacity down to the length.
    pub fn shrink_to_fit(&mut self) {
        self.repr.shrink_to_fit()
    }

    /// Removes and returns the element at `index`, moving the last element into its place.
    ///
    /// # Panics
    /// Panics if `index` is out of bounds.
    pub fn swap_remove(&mut self, index: UsizeId<TBrand>) -> TValue {
        self.repr.swap_remove(index.to_usize())
    }

    /// Shortens to `len`, dropping the rest. Keeps the contents if already shorter.
    pub fn truncate(&mut self, len: usize) {
        self.repr.truncate(len)
    }

    /// Creates an empty vec with room for `capacity` elements reserved up front.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }
}

impl<TBrand: ?Sized, TValue> AsMut<IdSlice<TBrand, TValue>> for IdVec<TBrand, TValue> {
    fn as_mut(&mut self) -> &mut IdSlice<TBrand, TValue> {
        self.as_mut_id_slice()
    }
}

impl<TBrand: ?Sized, TValue> AsMut<IdVec<TBrand, TValue>> for IdVec<TBrand, TValue> {
    fn as_mut(&mut self) -> &mut IdVec<TBrand, TValue> {
        self
    }
}

impl<TBrand: ?Sized, TValue> AsRef<IdSlice<TBrand, TValue>> for IdVec<TBrand, TValue> {
    fn as_ref(&self) -> &IdSlice<TBrand, TValue> {
        self.as_id_slice()
    }
}

impl<TBrand: ?Sized, TValue> AsRef<IdVec<TBrand, TValue>> for IdVec<TBrand, TValue> {
    fn as_ref(&self) -> &IdVec<TBrand, TValue> {
        self
    }
}

impl<TBrand: ?Sized, TValue> Borrow<IdSlice<TBrand, TValue>> for IdVec<TBrand, TValue> {
    fn borrow(&self) -> &IdSlice<TBrand, TValue> {
        IdSlice::from_slice(self.as_vec().borrow())
    }
}

impl<TBrand: ?Sized, TValue> BorrowMut<IdSlice<TBrand, TValue>> for IdVec<TBrand, TValue> {
    fn borrow_mut(&mut self) -> &mut IdSlice<TBrand, TValue> {
        IdSlice::from_mut_slice(self.as_mut_vec().borrow_mut())
    }
}

impl<TBrand: ?Sized, TValue: Clone> Clone for IdVec<TBrand, TValue> {
    fn clone(&self) -> Self {
        Self::from_vec(self.as_vec().clone())
    }
}

impl<TBrand: ?Sized, TValue: Debug> Debug for IdVec<TBrand, TValue> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_id_slice().fmt(f)
    }
}

impl<TBrand: ?Sized, TValue> Default for IdVec<TBrand, TValue> {
    fn default() -> Self {
        Self::from_vec(Default::default())
    }
}

impl<TBrand: ?Sized, TValue> Deref for IdVec<TBrand, TValue> {
    type Target = IdSlice<TBrand, TValue>;

    fn deref(&self) -> &Self::Target {
        IdSlice::from_slice(self.as_vec().deref())
    }
}

impl<TBrand: ?Sized, TValue> DerefMut for IdVec<TBrand, TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        IdSlice::from_mut_slice(self.as_mut_vec().deref_mut())
    }
}

impl<TBrand: ?Sized, TValue> Eq for IdVec<TBrand, TValue> where TValue: PartialEq {}

impl<'a, TBrand: ?Sized, TValue> Extend<&'a TValue> for IdVec<TBrand, TValue>
where
    TValue: Copy + 'a,
{
    fn extend<T: IntoIterator<Item = &'a TValue>>(&mut self, iter: T) {
        self.as_mut_vec().extend(iter)
    }
}

impl<TBrand: ?Sized, TValue> Extend<TValue> for IdVec<TBrand, TValue> {
    fn extend<T: IntoIterator<Item = TValue>>(&mut self, iter: T) {
        self.as_mut_vec().extend(iter)
    }
}

impl<TBrand: ?Sized, TValue> From<Vec<TValue>> for IdVec<TBrand, TValue> {
    fn from(value: Vec<TValue>) -> Self {
        Self::from_vec(value)
    }
}

impl<TBrand: ?Sized, TValue> FromIterator<TValue> for IdVec<TBrand, TValue> {
    fn from_iter<T: IntoIterator<Item = TValue>>(iter: T) -> Self {
        Self::from_vec(Vec::from_iter(iter))
    }
}

impl<TBrand: ?Sized, TValue> Hash for IdVec<TBrand, TValue>
where
    Vec<TValue>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_vec().hash(state)
    }

    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher,
    {
        // SAFETY: IdVec is #[repr(transparent)] over Vec<TValue>, so &[IdVec]
        // and &[Vec<TValue>] share a layout.
        let data = unsafe { transmute::<&[IdVec<TBrand, TValue>], &[Vec<TValue>]>(data) };
        <Vec<TValue>>::hash_slice(data, state)
    }
}

impl<TBrand: ?Sized, TValue, I: IdSliceIndex<IdSlice<TBrand, TValue>>> Index<I>
    for IdVec<TBrand, TValue>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.as_id_slice()[index]
    }
}

impl<TBrand: ?Sized, TValue, I: IdSliceIndex<IdSlice<TBrand, TValue>>> IndexMut<I>
    for IdVec<TBrand, TValue>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.as_mut_id_slice()[index]
    }
}

impl<'a, TBrand: ?Sized, TValue> IntoIterator for &'a IdVec<TBrand, TValue> {
    type Item = <&'a Vec<TValue> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<TValue> as IntoIterator>::IntoIter;

    #[allow(clippy::into_iter_on_ref)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_vec().into_iter()
    }
}

impl<'a, TBrand: ?Sized, TValue> IntoIterator for &'a mut IdVec<TBrand, TValue> {
    type Item = <&'a mut Vec<TValue> as IntoIterator>::Item;
    type IntoIter = <&'a mut Vec<TValue> as IntoIterator>::IntoIter;

    #[allow(clippy::into_iter_on_ref)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_vec().into_iter()
    }
}

impl<TBrand: ?Sized, TValue> IntoIterator for IdVec<TBrand, TValue> {
    type Item = <Vec<TValue> as IntoIterator>::Item;
    type IntoIter = <Vec<TValue> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<TBrand: ?Sized, TValue: Ord> Ord for IdVec<TBrand, TValue> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_vec().cmp(other.as_vec())
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_vec(self.into_vec().max(other.into_vec()))
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Self::from_vec(self.into_vec().min(other.into_vec()))
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        Self::from_vec(self.into_vec().clamp(min.into_vec(), max.into_vec()))
    }
}

impl<TBrand: ?Sized, TValueA, TValueB> PartialEq<&IdSlice<TBrand, TValueB>>
    for IdVec<TBrand, TValueA>
where
    TValueA: PartialEq<TValueB>,
{
    fn eq(&self, other: &&IdSlice<TBrand, TValueB>) -> bool {
        self.as_id_slice().eq(*other)
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &&IdSlice<TBrand, TValueB>) -> bool {
        self.as_id_slice().ne(*other)
    }
}

impl<TBrand: ?Sized, TValueA, TValueB, const N: usize> PartialEq<&IdArray<TBrand, TValueB, N>>
    for IdVec<TBrand, TValueA>
where
    TValueA: PartialEq<TValueB>,
{
    fn eq(&self, other: &&IdArray<TBrand, TValueB, N>) -> bool {
        self.as_id_slice().eq(other.as_id_slice())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &&IdArray<TBrand, TValueB, N>) -> bool {
        self.as_id_slice().ne(other.as_id_slice())
    }
}

impl<TBrand: ?Sized, TValueA, TValueB> PartialEq<&mut IdSlice<TBrand, TValueB>>
    for IdVec<TBrand, TValueA>
where
    TValueA: PartialEq<TValueB>,
{
    fn eq(&self, other: &&mut IdSlice<TBrand, TValueB>) -> bool {
        self.as_id_slice().eq(*other)
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &&mut IdSlice<TBrand, TValueB>) -> bool {
        self.as_id_slice().ne(*other)
    }
}

impl<TBrand: ?Sized, TValueA, TValueB> PartialEq<IdSlice<TBrand, TValueB>>
    for IdVec<TBrand, TValueA>
where
    TValueA: PartialEq<TValueB>,
{
    fn eq(&self, other: &IdSlice<TBrand, TValueB>) -> bool {
        self.as_id_slice().eq(other)
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdSlice<TBrand, TValueB>) -> bool {
        self.as_id_slice().ne(other)
    }
}

impl<TBrand: ?Sized, TValueA, TValueB, const N: usize> PartialEq<IdArray<TBrand, TValueB, N>>
    for IdVec<TBrand, TValueA>
where
    TValueA: PartialEq<TValueB>,
{
    fn eq(&self, other: &IdArray<TBrand, TValueB, N>) -> bool {
        self.as_id_slice().eq(other.as_id_slice())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdArray<TBrand, TValueB, N>) -> bool {
        self.as_id_slice().ne(other.as_id_slice())
    }
}

impl<TBrand: ?Sized, TValueA, TValueB> PartialEq<IdVec<TBrand, TValueB>> for IdVec<TBrand, TValueA>
where
    TValueA: PartialEq<TValueB>,
{
    fn eq(&self, other: &IdVec<TBrand, TValueB>) -> bool {
        self.as_vec().eq(other.as_vec())
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &IdVec<TBrand, TValueB>) -> bool {
        self.as_vec().ne(other.as_vec())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<TBrand: ?Sized, TValue: PartialOrd> PartialOrd for IdVec<TBrand, TValue> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_vec().partial_cmp(other.as_vec())
    }

    fn lt(&self, other: &Self) -> bool {
        self.as_vec().lt(other.as_vec())
    }

    fn le(&self, other: &Self) -> bool {
        self.as_vec().le(other.as_vec())
    }

    fn gt(&self, other: &Self) -> bool {
        self.as_vec().gt(other.as_vec())
    }

    fn ge(&self, other: &Self) -> bool {
        self.as_vec().ge(other.as_vec())
    }
}
