use crate::{
    Id, Scalar,
    soa::{IdColumns, IdStructIter},
};
use std::marker::PhantomData;

/// Iterates `(id, row)` in a pool's iteration order with rows read through a
/// shared borrow. Created by `iter` on a view or an [`IdList`](super::IdList).
pub struct IdStructViewIter<'r, TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar = u32> {
    ids: IdStructIter<'r, TNum::Id<TBrand>>,

    raw: TColumns::RawRef,

    marker: PhantomData<&'r TColumns>,
}

impl<'r, TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar>
    IdStructViewIter<'r, TBrand, TColumns, TNum>
{
    /// # Safety
    /// `raw` comes from columns in sync with the pool `ids` iterates, and the
    /// columns stay unchanged for `'r`.
    pub(super) unsafe fn new(
        ids: IdStructIter<'r, TNum::Id<TBrand>>,
        raw: TColumns::RawRef,
    ) -> Self {
        Self {
            ids,
            raw,
            marker: PhantomData,
        }
    }
}

impl<TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar> Clone
    for IdStructViewIter<'_, TBrand, TColumns, TNum>
{
    fn clone(&self) -> Self {
        Self {
            ids: self.ids.clone(),
            raw: self.raw,
            marker: PhantomData,
        }
    }
}

impl<'r, TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar> Iterator
    for IdStructViewIter<'r, TBrand, TColumns, TNum>
{
    type Item = (TNum::Id<TBrand>, TColumns::Ref<'r>);

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.ids.next()?;

        // SAFETY: by `new`'s contract, a retained id has a value in every
        // column.
        Some((id, unsafe { TColumns::row_ref(self.raw, id.to_usize_id()) }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ids.size_hint()
    }
}

impl<TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar> DoubleEndedIterator
    for IdStructViewIter<'_, TBrand, TColumns, TNum>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let id = self.ids.next_back()?;

        // SAFETY: see `next`.
        Some((id, unsafe { TColumns::row_ref(self.raw, id.to_usize_id()) }))
    }
}

impl<TBrand: ?Sized, TColumns: IdColumns<TBrand>, TNum: Scalar> ExactSizeIterator
    for IdStructViewIter<'_, TBrand, TColumns, TNum>
{
}
