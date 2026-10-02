use crate::{
    Scalar,
    soa::{
        IdColumns, IdField, IdRemap, IdStruct, IdStructView, IdStructViewIter, IdStructViewIterMut,
        IdStructViewMut,
    },
};
use std::fmt::{self, Debug};

/// A list of values under stable branded ids: an [`IdStruct`] that owns one
/// value per id.
///
/// Each value keeps the id [`retain`](Self::retain) hands out until it is
/// released. A released id is recycled. The values list in the pool's
/// iteration order, which [`move_to`](Self::move_to) and
/// [`set_order`](Self::set_order) rearrange. The list owns its pool and its
/// one column, so it keeps them in sync without `unsafe`. Several columns over
/// one pool go through [`IdStruct::view_mut`] instead.
pub struct IdList<TBrand: ?Sized, TValue, TNum: Scalar = u32> {
    ids: IdStruct<TBrand, TNum>,

    values: IdField<TBrand, TValue>,
}

impl<TBrand: ?Sized, TValue, TNum: Scalar> IdList<TBrand, TValue, TNum> {
    /// Creates an empty list.
    pub const fn new() -> Self {
        Self {
            ids: IdStruct::new(),
            values: IdField::new(),
        }
    }

    /// Removes every value and resets the pool to empty.
    pub fn clear(&mut self) {
        self.view_mut().clear();
    }

    /// Whether both lists hold the same ids in the same order with equal
    /// values. Released ids do not count.
    pub fn eq_entries(&self, other: &Self) -> bool
    where
        TValue: PartialEq,
    {
        self.iter().eq(other.iter())
    }

    /// Whether both lists hold the same entries and queue the same released
    /// ids in the same order for reuse.
    pub fn eq_state(&self, other: &Self) -> bool
    where
        TValue: PartialEq,
    {
        self.ids.eq_state(&other.ids) && self.eq_values(other)
    }

    /// Whether both lists hold equal values in the same order, whatever their
    /// ids.
    pub fn eq_values(&self, other: &Self) -> bool
    where
        TValue: PartialEq,
    {
        let values = self.iter().map(|(_, value)| value);

        let other_values = other.iter().map(|(_, value)| value);

        values.eq(other_values)
    }

    /// Compacts the list so its ids become `0..len`, as [`IdStruct::gc`].
    /// Returns the remap for ids held elsewhere.
    pub fn gc(&mut self) -> IdRemap<TBrand, TNum> {
        self.view_mut().gc()
    }

    /// The value at `id`, or `None` when the list does not hold `id`.
    pub fn get(&self, id: TNum::Id<TBrand>) -> Option<&TValue> {
        self.view().get(id)
    }

    /// The value at `id`, writable, or `None` when the list does not hold
    /// `id`.
    pub fn get_mut(&mut self, id: TNum::Id<TBrand>) -> Option<&mut TValue> {
        self.view_mut().into_mut(id)
    }

    /// The pool of the list's ids, for order and membership queries.
    pub fn ids(&self) -> &IdStruct<TBrand, TNum> {
        &self.ids
    }

    /// Whether the list holds no values.
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Iterates `(id, value)` in the list's order.
    pub fn iter(&self) -> IdStructViewIter<'_, TBrand, &IdField<TBrand, TValue>, TNum> {
        // SAFETY: the list keeps its column in sync with its pool, and the
        // borrow keeps both unchanged.
        unsafe { IdStructViewIter::new(self.ids.iter(), (&self.values).raw_ref()) }
    }

    /// Iterates `(id, value)` in the list's order with writable values.
    pub fn iter_mut(
        &mut self,
    ) -> IdStructViewIterMut<'_, TBrand, &mut IdField<TBrand, TValue>, TNum> {
        self.view_mut().into_iter()
    }

    /// The number of values.
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// Moves `id` to position `index` in the order, as [`IdStruct::move_to`].
    ///
    /// # Panics
    /// Panics if the list does not hold `id` or if `index` is at or past
    /// [`len`](Self::len).
    pub fn move_to(&mut self, id: TNum::Id<TBrand>, index: usize) {
        self.ids.move_to(id, index);
    }

    /// Removes the value at `id` and hands it back, or `None` when the list
    /// does not hold `id`. The last value takes its place in the order, as
    /// [`IdStruct::release`].
    pub fn release(&mut self, id: TNum::Id<TBrand>) -> Option<TValue> {
        self.view_mut().release(id)
    }

    /// Removes the value at `id` and hands it back, or `None` when the list
    /// does not hold `id`. The values after it keep their order, as
    /// [`IdStruct::release_stable`].
    pub fn release_stable(&mut self, id: TNum::Id<TBrand>) -> Option<TValue> {
        self.view_mut().release_stable(id)
    }

    /// Like [`release_stable`](Self::release_stable), but also clobbers the
    /// value's slot with zeros, as [`IdField::take_zeroed`].
    pub fn release_stable_zeroed(&mut self, id: TNum::Id<TBrand>) -> Option<TValue> {
        self.view_mut().release_stable_zeroed(id)
    }

    /// Like [`release`](Self::release), but also clobbers the value's slot
    /// with zeros, as [`IdField::take_zeroed`].
    pub fn release_zeroed(&mut self, id: TNum::Id<TBrand>) -> Option<TValue> {
        self.view_mut().release_zeroed(id)
    }

    /// Adds `value` under a newly retained id at the end of the order.
    pub fn retain(&mut self, value: TValue) -> TNum::Id<TBrand> {
        self.view_mut().retain(value)
    }

    /// Rewrites the order to `new_order`, as [`IdStruct::set_order`].
    ///
    /// # Panics
    /// Panics if `new_order` does not list every id the list holds exactly
    /// once.
    pub fn set_order(&mut self, new_order: &[TNum::Id<TBrand>]) {
        self.ids.set_order(new_order);
    }

    /// Moves `id` to position `index` in the order, or `None` when the list
    /// does not hold `id` or `index` is at or past [`len`](Self::len).
    pub fn try_move_to(&mut self, id: TNum::Id<TBrand>, index: usize) -> Option<()> {
        self.ids.try_move_to(id, index)
    }

    /// Rewrites the order to `new_order`, or `None` when `new_order` does not
    /// list every id the list holds exactly once.
    pub fn try_set_order(&mut self, new_order: &[TNum::Id<TBrand>]) -> Option<()> {
        self.ids.try_set_order(new_order)
    }

    /// A shared view of the list, whose rows can outlive the view.
    pub fn view(&self) -> IdStructView<'_, TBrand, &IdField<TBrand, TValue>, TNum> {
        // SAFETY: the list keeps its column in sync with its pool.
        unsafe { self.ids.view(&self.values) }
    }

    /// A mutable view of the list.
    pub fn view_mut(&mut self) -> IdStructViewMut<'_, TBrand, &mut IdField<TBrand, TValue>, TNum> {
        // SAFETY: the list keeps its column in sync with its pool, and that
        // column is the pool's only one.
        unsafe { self.ids.view_mut(&mut self.values) }
    }
}

impl<TBrand: ?Sized, TValue: Clone, TNum: Scalar> Clone for IdList<TBrand, TValue, TNum> {
    fn clone(&self) -> Self {
        Self {
            ids: self.ids.clone(),

            // SAFETY: the list keeps its column in sync with its pool.
            values: unsafe { self.values.clone_retained(&self.ids) },
        }
    }
}

impl<TBrand: ?Sized, TValue: Debug, TNum: Scalar> Debug for IdList<TBrand, TValue, TNum>
where
    TNum::Id<TBrand>: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<TBrand: ?Sized, TValue, TNum: Scalar> Default for IdList<TBrand, TValue, TNum> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TBrand: ?Sized, TValue, TNum: Scalar> Drop for IdList<TBrand, TValue, TNum> {
    fn drop(&mut self) {
        // SAFETY: the list keeps its column in sync with its pool.
        unsafe { self.values.clear(&self.ids) };
    }
}

impl<TBrand: ?Sized, TValue, TNum: Scalar> Extend<TValue> for IdList<TBrand, TValue, TNum> {
    fn extend<TIter: IntoIterator<Item = TValue>>(&mut self, values: TIter) {
        for value in values {
            self.retain(value);
        }
    }
}

impl<TBrand: ?Sized, TValue, TNum: Scalar> FromIterator<TValue> for IdList<TBrand, TValue, TNum> {
    fn from_iter<TIter: IntoIterator<Item = TValue>>(values: TIter) -> Self {
        let mut list = Self::new();

        list.extend(values);

        list
    }
}

impl<'r, TBrand: ?Sized, TValue, TNum: Scalar> IntoIterator for &'r IdList<TBrand, TValue, TNum> {
    type Item = (TNum::Id<TBrand>, &'r TValue);

    type IntoIter = IdStructViewIter<'r, TBrand, &'r IdField<TBrand, TValue>, TNum>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'r, TBrand: ?Sized, TValue, TNum: Scalar> IntoIterator
    for &'r mut IdList<TBrand, TValue, TNum>
{
    type Item = (TNum::Id<TBrand>, &'r mut TValue);

    type IntoIter = IdStructViewIterMut<'r, TBrand, &'r mut IdField<TBrand, TValue>, TNum>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
