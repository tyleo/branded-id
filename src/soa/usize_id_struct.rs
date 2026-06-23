use crate::{
    IdVec, UsizeId,
    ext::UsizeExt,
    soa::{UsizeIdStructIter, UsizeIdStructRawParts},
};

/// An id pool that hands out and recycles typed integer handles.
///
/// Ids are allocated with [`retain`](Self::retain) and recycled with
/// [`release`](Self::release). Retained ids are kept in a packed `live`
/// list; releasing swap-removes from that list so iteration only ever
/// visits retained ids (in swap-remove order, not sorted).
pub struct UsizeIdStruct<TMarker: ?Sized> {
    /// The ids currently retained, packed.
    live: Vec<UsizeId<TMarker>>,
    /// Per-id: the index of the id in `live` plus 1. 0 means "not retained".
    live_index_plus_one: IdVec<TMarker, usize>,
    /// Released ids available for reuse (LIFO).
    free: Vec<UsizeId<TMarker>>,
}

impl<TMarker: ?Sized> UsizeIdStruct<TMarker> {
    pub const fn new() -> Self {
        Self {
            live: Vec::new(),
            live_index_plus_one: IdVec::new(),
            free: Vec::new(),
        }
    }

    /// Exposes the internal lists for advanced usage.
    pub fn as_raw_parts(&self) -> UsizeIdStructRawParts<'_, TMarker> {
        UsizeIdStructRawParts {
            live: &self.live,
            live_index_plus_one: &self.live_index_plus_one,
            free: &self.free,
        }
    }

    /// Releases every retained id and resets the pool to empty.
    pub fn clear(&mut self) {
        self.live.clear();
        self.live_index_plus_one.as_mut_vec().clear();
        self.free.clear();
    }

    /// The number of ids currently retained from this pool.
    pub fn count(&self) -> usize {
        self.live.len()
    }

    pub fn is_retained(&self, id: UsizeId<TMarker>) -> bool {
        id.to_usize() < self.live_index_plus_one.len() && self.live_index_plus_one[id] != 0
    }

    /// Peeks at the next id [`retain`](Self::retain) would return, without
    /// actually retaining it.
    pub fn peek_next(&self) -> UsizeId<TMarker> {
        match self.free.last() {
            Some(&id) => id,
            None => self.peek_next_fresh(),
        }
    }

    /// Peeks at the next id that would be freshly allocated, ignoring the
    /// free list.
    pub fn peek_next_fresh(&self) -> UsizeId<TMarker> {
        self.live_index_plus_one.len().to_usize_id()
    }

    pub fn retain(&mut self) -> UsizeId<TMarker> {
        // Recycle a freed id if one is available; otherwise allocate a
        // brand-new one (which also grows the reverse-index list).
        let id = match self.free.pop() {
            Some(free_id) => free_id,
            None => self.live_index_plus_one.push(0),
        };

        self.live.push(id);
        // Store index + 1 so that 0 can mean "not retained".
        self.live_index_plus_one[id] = self.live.len();

        id
    }

    pub fn release(&mut self, id: UsizeId<TMarker>) {
        let live_index_plus_one = self.live_index_plus_one[id];

        // Mark the id as no longer retained and push it onto the free list
        // for future recycling.
        self.live_index_plus_one[id] = 0;
        self.free.push(id);

        // Swap-remove: move the last live id into the slot vacated by the
        // released id to keep `live` packed.
        let last_id = self.live.pop().expect("released an id from an empty pool");
        if id == last_id {
            return;
        }

        self.live_index_plus_one[last_id] = live_index_plus_one;
        self.live[live_index_plus_one - 1] = last_id;
    }
}

impl<TMarker: ?Sized> Default for UsizeIdStruct<TMarker> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, TMarker: ?Sized> IntoIterator for &'a UsizeIdStruct<TMarker> {
    type Item = UsizeId<TMarker>;
    type IntoIter = UsizeIdStructIter<'a, TMarker>;

    fn into_iter(self) -> Self::IntoIter {
        UsizeIdStructIter::from_live(&self.live)
    }
}
