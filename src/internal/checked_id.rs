use crate::{Id, UsizeId};

/// The id at `index`, or `None` if `index` does not fit the id width.
pub fn checked_id<TId: Id>(index: usize) -> Option<TId> {
    let id = TId::from_usize_id(UsizeId::from_usize(index));
    (id.to_usize_id().to_usize() == index).then_some(id)
}
