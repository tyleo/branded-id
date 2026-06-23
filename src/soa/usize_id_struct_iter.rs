use crate::UsizeId;
use std::slice;

/// Iterates the ids currently retained by a [`UsizeIdStruct`], in the
/// order they appear in the packed `live` list (swap-remove order).
///
/// [`UsizeIdStruct`]: super::UsizeIdStruct
pub struct UsizeIdStructIter<'a, TMarker: ?Sized> {
    live: slice::Iter<'a, UsizeId<TMarker>>,
}

impl<'a, TMarker: ?Sized> UsizeIdStructIter<'a, TMarker> {
    pub fn from_live(live: &'a [UsizeId<TMarker>]) -> Self {
        Self { live: live.iter() }
    }
}

impl<TMarker: ?Sized> Iterator for UsizeIdStructIter<'_, TMarker> {
    type Item = UsizeId<TMarker>;

    fn next(&mut self) -> Option<Self::Item> {
        self.live.next().copied()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.live.size_hint()
    }
}

impl<TMarker: ?Sized> DoubleEndedIterator for UsizeIdStructIter<'_, TMarker> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.live.next_back().copied()
    }
}

impl<TMarker: ?Sized> ExactSizeIterator for UsizeIdStructIter<'_, TMarker> {}
