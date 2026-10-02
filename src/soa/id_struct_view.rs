use crate::{
    Id, Scalar,
    soa::{IdColumns, IdStruct, IdStructViewIter, IdStructViewIterMut},
};
use std::fmt::{self, Debug};

/// An [`IdStruct`] beside some of its columns, read and written by row.
///
/// A column alone cannot tell which of its slots hold values, so its reads are
/// `unsafe`. A view checks each id against the pool and reads and writes
/// safely. Building one with [`IdStruct::view`] is the single unsafe step: the
/// caller vouches that every column is in sync with the pool. No id comes or
/// goes while the view borrows the pool.
///
/// `TColumns` is one `&IdField` or `&mut IdField`, or a tuple of them. See
/// [`IdColumns`] for how each reads.
pub struct IdStructView<'a, TBrand: ?Sized, TColumns, TNum: Scalar = u32> {
    ids: &'a IdStruct<TBrand, TNum>,

    columns: TColumns,
}

impl<'a, TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar>
    IdStructView<'a, TBrand, TColumns, TNum>
{
    /// # Safety
    /// See [`IdStruct::view`].
    pub(super) unsafe fn new(ids: &'a IdStruct<TBrand, TNum>, columns: TColumns) -> Self {
        Self { ids, columns }
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

    /// The row at `id` with its mutable columns writable, or `None` when the
    /// pool does not retain `id`.
    pub fn get_mut(&mut self, id: TNum::Id<TBrand>) -> Option<TColumns::Mut<'_>> {
        if !self.ids.is_retained(id) {
            return None;
        }

        // SAFETY: the columns are in sync with the pool, and the borrow hands
        // out only this row.
        Some(unsafe { TColumns::row_mut(self.columns.raw_mut(), id.to_usize_id()) })
    }

    /// The pool keying the columns.
    pub fn ids(&self) -> &'a IdStruct<TBrand, TNum> {
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

    /// Iterates `(id, row)` in the pool's iteration order with the mutable
    /// columns writable.
    pub fn iter_mut(&mut self) -> IdStructViewIterMut<'_, TBrand, TColumns, TNum> {
        // SAFETY: the columns are in sync with the pool, and the borrow keeps
        // everything else off them.
        unsafe { IdStructViewIterMut::new(self.ids.iter(), self.columns.raw_mut()) }
    }

    /// The number of ids the pool retains, one row each.
    pub fn len(&self) -> usize {
        self.ids.len()
    }
}

impl<TBrand: ?Sized, TColumns: Copy, TNum: Scalar> Clone
    for IdStructView<'_, TBrand, TColumns, TNum>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<TBrand: ?Sized, TColumns: Copy, TNum: Scalar> Copy
    for IdStructView<'_, TBrand, TColumns, TNum>
{
}

impl<TBrand: ?Sized, TColumns, TNum: Scalar> Debug for IdStructView<'_, TBrand, TColumns, TNum>
where
    TNum::Id<TBrand>: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // A row is a tuple of references whose `Debug` would need a bound per
        // lifetime, so only the pool is shown.
        f.debug_struct("IdStructView")
            .field("ids", self.ids)
            .finish_non_exhaustive()
    }
}

impl<'a, TBrand: ?Sized, TColumns: IdColumns<TBrand> + 'a, TNum: Scalar> IntoIterator
    for IdStructView<'a, TBrand, TColumns, TNum>
{
    type Item = (TNum::Id<TBrand>, TColumns::Mut<'a>);

    type IntoIter = IdStructViewIterMut<'a, TBrand, TColumns, TNum>;

    fn into_iter(mut self) -> Self::IntoIter {
        // SAFETY: the columns are in sync with the pool, and consuming the
        // view keeps everything else off them.
        unsafe { IdStructViewIterMut::new(self.ids.iter(), self.columns.raw_mut()) }
    }
}

impl<'r, TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar> IntoIterator
    for &'r IdStructView<'_, TBrand, TColumns, TNum>
{
    type Item = (TNum::Id<TBrand>, TColumns::Ref<'r>);

    type IntoIter = IdStructViewIter<'r, TBrand, TColumns, TNum>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'r, TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar> IntoIterator
    for &'r mut IdStructView<'_, TBrand, TColumns, TNum>
{
    type Item = (TNum::Id<TBrand>, TColumns::Mut<'r>);

    type IntoIter = IdStructViewIterMut<'r, TBrand, TColumns, TNum>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
