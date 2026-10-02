use crate::{
    Id, Scalar,
    soa::{IdColumns, IdStructIter},
};
use std::marker::PhantomData;

/// Iterates `(id, row)` in a pool's iteration order with rows read through a
/// mutable borrow. Created by `iter_mut` on a view or an
/// [`IdList`](super::IdList), and by consuming a view.
pub struct IdStructViewIterMut<'r, TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar = u32>
{
    ids: IdStructIter<'r, TNum::Id<TBrand>>,

    raw: TColumns::RawMut,

    marker: PhantomData<&'r mut TColumns>,
}

impl<'r, TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar>
    IdStructViewIterMut<'r, TBrand, TColumns, TNum>
{
    /// # Safety
    /// `raw` comes from columns in sync with the pool `ids` iterates, and
    /// nothing else reaches the columns for `'r`.
    pub(super) unsafe fn new(
        ids: IdStructIter<'r, TNum::Id<TBrand>>,
        raw: TColumns::RawMut,
    ) -> Self {
        Self {
            ids,
            raw,
            marker: PhantomData,
        }
    }
}

impl<'r, TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar> Iterator
    for IdStructViewIterMut<'r, TBrand, TColumns, TNum>
{
    type Item = (TNum::Id<TBrand>, TColumns::Mut<'r>);

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.ids.next()?;

        // SAFETY: by `new`'s contract, a retained id has a value in every
        // column, and the pool yields each id once, so no two rows alias.
        Some((id, unsafe { TColumns::row_mut(self.raw, id.to_usize_id()) }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ids.size_hint()
    }
}

impl<TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar> DoubleEndedIterator
    for IdStructViewIterMut<'_, TBrand, TColumns, TNum>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let id = self.ids.next_back()?;

        // SAFETY: see `next`.
        Some((id, unsafe { TColumns::row_mut(self.raw, id.to_usize_id()) }))
    }
}

impl<TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar> ExactSizeIterator
    for IdStructViewIterMut<'_, TBrand, TColumns, TNum>
{
}
