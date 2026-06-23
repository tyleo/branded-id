use crate::{IdVec, UsizeId};

/// Borrowed views of the internal lists of a [`UsizeIdStruct`], as
/// returned by [`UsizeIdStruct::as_raw_parts`].
///
/// [`UsizeIdStruct`]: super::UsizeIdStruct
/// [`UsizeIdStruct::as_raw_parts`]: super::UsizeIdStruct::as_raw_parts
pub struct UsizeIdStructRawParts<'a, TMarker: ?Sized> {
    /// The ids currently retained, packed.
    pub live: &'a [UsizeId<TMarker>],
    /// Per-id: the index of the id in `live` plus 1. 0 means "not retained".
    pub live_index_plus_one: &'a IdVec<TMarker, usize>,
    /// Released ids available for reuse (LIFO).
    pub free: &'a [UsizeId<TMarker>],
}
