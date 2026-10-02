use crate::{
    Id, Scalar,
    soa::{IdColumnsMut, IdRemap, IdStruct, IdStructViewIter, IdStructViewIterMut},
};
use std::fmt::{self, Debug};

/// An [`IdStruct`] beside every one of its columns, all mutable. Rows can be
/// added and removed as well as read and written.
///
/// The mutable counterpart of [`IdStructView`](super::IdStructView), built
/// with [`IdStruct::view_mut`]. Adding or removing a row touches the pool and
/// each column together, which keeps them in sync only because the view holds
/// every column. The caller vouches for that when building it.
pub struct IdStructViewMut<'a, TBrand: ?Sized, TColumns, TNum: Scalar = u32> {
    ids: &'a mut IdStruct<TBrand, TNum>,

    columns: TColumns,
}

impl<'a, TBrand: ?Sized, TColumns: IdColumnsMut<TBrand>, TNum: Scalar>
    IdStructViewMut<'a, TBrand, TColumns, TNum>
{
    /// # Safety
    /// See [`IdStruct::view_mut`].
    pub(super) unsafe fn new(ids: &'a mut IdStruct<TBrand, TNum>, columns: TColumns) -> Self {
        Self { ids, columns }
    }

    /// Removes every row, drops its values, and resets the pool to empty.
    pub fn clear(&mut self) {
        // SAFETY: the columns are in sync with the pool.
        unsafe { self.columns.clear(self.ids) };

        self.ids.clear();
    }

    /// Compacts the pool and every column so the retained ids become
    /// `0..len`, as [`IdStruct::gc`]. Returns the remap for ids held elsewhere.
    pub fn gc(&mut self) -> IdRemap<TBrand, TNum> {
        let remap = self.ids.gc();

        // SAFETY: the columns are every column of the pool, in sync with its
        // layout before the gc.
        unsafe { self.columns.gc(&remap) };

        remap
    }

    /// The row at `id`, or `None` when the pool does not retain `id`.
    pub fn get(&self, id: TNum::Id<TBrand>) -> Option<TColumns::Ref<'_>> {
        if !self.ids.is_retained(id) {
            return None;
        }

        // SAFETY: the columns are in sync with the pool, so a retained id has
        // a value in each.
        Some(unsafe { TColumns::row_ref(self.columns.raw_ref(), id.to_usize_id()) })
    }

    /// The row at `id`, writable, or `None` when the pool does not retain
    /// `id`.
    pub fn get_mut(&mut self, id: TNum::Id<TBrand>) -> Option<TColumns::Mut<'_>> {
        if !self.ids.is_retained(id) {
            return None;
        }

        // SAFETY: the columns are in sync with the pool, and the borrow hands
        // out only this row.
        Some(unsafe { TColumns::row_mut(self.columns.raw_mut(), id.to_usize_id()) })
    }

    /// The pool keying the columns.
    pub fn ids(&self) -> &IdStruct<TBrand, TNum> {
        self.ids
    }

    /// Consumes the view and returns the row at `id` for the view's whole
    /// lifetime, or `None` when the pool does not retain `id`.
    pub fn into_mut(mut self, id: TNum::Id<TBrand>) -> Option<TColumns::Mut<'a>>
    where
        TColumns: 'a,
    {
        if !self.ids.is_retained(id) {
            return None;
        }

        // SAFETY: the columns are in sync with the pool, and consuming the
        // view hands out only this row.
        Some(unsafe { TColumns::row_mut(self.columns.raw_mut(), id.to_usize_id()) })
    }

    /// Whether the pool retains no ids.
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Iterates `(id, row)` in the pool's iteration order.
    pub fn iter(&self) -> IdStructViewIter<'_, TBrand, TColumns, TNum> {
        // SAFETY: the columns are in sync with the pool, and the borrow keeps
        // them unchanged.
        unsafe { IdStructViewIter::new(self.ids.iter(), self.columns.raw_ref()) }
    }

    /// Iterates writable `(id, row)` pairs in the pool's iteration order.
    pub fn iter_mut(&mut self) -> IdStructViewIterMut<'_, TBrand, TColumns, TNum> {
        // SAFETY: the columns are in sync with the pool, and the borrow keeps
        // everything else off them.
        unsafe { IdStructViewIterMut::new(self.ids.iter(), self.columns.raw_mut()) }
    }

    /// The number of ids the pool retains, one row each.
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// Removes the row at `id` and hands its values back, or `None` when the
    /// pool does not retain `id`. The last row takes its place in the order,
    /// as [`IdStruct::release`].
    pub fn release(&mut self, id: TNum::Id<TBrand>) -> Option<TColumns::Values> {
        if !self.ids.is_retained(id) {
            return None;
        }

        // SAFETY: the columns are in sync with the pool, so a retained id has
        // a value in each, taken before the id goes.
        let values = unsafe { self.columns.take(id.to_usize_id()) };

        self.ids.release(id);

        Some(values)
    }

    /// Removes the row at `id` and hands its values back, or `None` when the
    /// pool does not retain `id`. The rows after it keep their order, as
    /// [`IdStruct::release_stable`].
    pub fn release_stable(&mut self, id: TNum::Id<TBrand>) -> Option<TColumns::Values> {
        if !self.ids.is_retained(id) {
            return None;
        }

        // SAFETY: the columns are in sync with the pool, so a retained id has
        // a value in each, taken before the id goes.
        let values = unsafe { self.columns.take(id.to_usize_id()) };

        self.ids.release_stable(id);

        Some(values)
    }

    /// Like [`release_stable`](Self::release_stable), but also clobbers the
    /// row's slots with zeros, as
    /// [`IdField::take_zeroed`](super::IdField::take_zeroed).
    pub fn release_stable_zeroed(&mut self, id: TNum::Id<TBrand>) -> Option<TColumns::Values> {
        if !self.ids.is_retained(id) {
            return None;
        }

        // SAFETY: the columns are in sync with the pool, so a retained id has
        // a value in each, taken before the id goes.
        let values = unsafe { self.columns.take_zeroed(id.to_usize_id()) };

        self.ids.release_stable(id);

        Some(values)
    }

    /// Like [`release`](Self::release), but also clobbers the row's slots with
    /// zeros, as [`IdField::take_zeroed`](super::IdField::take_zeroed).
    pub fn release_zeroed(&mut self, id: TNum::Id<TBrand>) -> Option<TColumns::Values> {
        if !self.ids.is_retained(id) {
            return None;
        }

        // SAFETY: the columns are in sync with the pool, so a retained id has
        // a value in each, taken before the id goes.
        let values = unsafe { self.columns.take_zeroed(id.to_usize_id()) };

        self.ids.release(id);

        Some(values)
    }

    /// Adds a row holding `values` under a newly retained id at the end of the
    /// order.
    pub fn retain(&mut self, values: TColumns::Values) -> TNum::Id<TBrand> {
        let id = self.ids.retain();

        self.columns.retain(id.to_usize_id(), values);

        id
    }
}

impl<TBrand: ?Sized, TColumns, TNum: Scalar> Debug for IdStructViewMut<'_, TBrand, TColumns, TNum>
where
    TNum::Id<TBrand>: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // A row is a tuple of references whose `Debug` would need a bound per
        // lifetime, so only the pool is shown.
        f.debug_struct("IdStructViewMut")
            .field("ids", &self.ids)
            .finish_non_exhaustive()
    }
}

impl<'a, TBrand: ?Sized, TColumns: IdColumnsMut<TBrand> + 'a, TNum: Scalar> IntoIterator
    for IdStructViewMut<'a, TBrand, TColumns, TNum>
{
    type Item = (TNum::Id<TBrand>, TColumns::Mut<'a>);

    type IntoIter = IdStructViewIterMut<'a, TBrand, TColumns, TNum>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { ids, mut columns } = self;

        let ids: &'a IdStruct<TBrand, TNum> = ids;

        // SAFETY: the columns are in sync with the pool, and consuming the
        // view keeps everything else off them.
        unsafe { IdStructViewIterMut::new(ids.iter(), columns.raw_mut()) }
    }
}

impl<'r, TBrand: ?Sized, TColumns: IdColumnsMut<TBrand>, TNum: Scalar> IntoIterator
    for &'r IdStructViewMut<'_, TBrand, TColumns, TNum>
{
    type Item = (TNum::Id<TBrand>, TColumns::Ref<'r>);

    type IntoIter = IdStructViewIter<'r, TBrand, TColumns, TNum>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'r, TBrand: ?Sized, TColumns: IdColumnsMut<TBrand>, TNum: Scalar> IntoIterator
    for &'r mut IdStructViewMut<'_, TBrand, TColumns, TNum>
{
    type Item = (TNum::Id<TBrand>, TColumns::Mut<'r>);

    type IntoIter = IdStructViewIterMut<'r, TBrand, TColumns, TNum>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
