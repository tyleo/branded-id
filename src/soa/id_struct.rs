use crate::{
    Id, IdVec,
    soa::{IdStructIter, IdStructRawParts},
};

/// An id pool that hands out and recycles typed integer handles.
///
/// Ids are allocated with [`retain`](Self::retain) and recycled with
/// [`release`](Self::release). Retained ids are kept in a packed `live`
/// list; releasing swap-removes from that list so iteration only ever
/// visits retained ids.
///
/// The pool is generic over the id type `TId`: see
/// [`U32IdStruct`](super::U32IdStruct) for the common `u32`-keyed
/// specialization.
pub struct IdStruct<TId: Id> {
    /// The ids currently retained, packed.
    live: Vec<TId>,
    /// Per-id: the index of the id in `live` plus 1. 0 means "not retained".
    live_index_plus_one: IdVec<TId::Marker, usize>,
    /// Released ids available for reuse (LIFO).
    free: Vec<TId>,
}

impl<TId: Id> IdStruct<TId> {
    pub const fn new() -> Self {
        Self {
            live: Vec::new(),
            live_index_plus_one: IdVec::new(),
            free: Vec::new(),
        }
    }

    /// Exposes the internal lists for advanced usage.
    pub fn as_raw_parts(&self) -> IdStructRawParts<'_, TId> {
        IdStructRawParts {
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

    pub fn is_retained(&self, id: TId) -> bool {
        let id = id.to_usize_id();
        id.to_usize() < self.live_index_plus_one.len() && self.live_index_plus_one[id] != 0
    }

    /// Peeks at the next id [`retain`](Self::retain) would return, without
    /// actually retaining it.
    pub fn peek_next(&self) -> TId {
        match self.free.last() {
            Some(&id) => id,
            None => self.peek_next_fresh(),
        }
    }

    /// Peeks at the next id that would be freshly allocated, ignoring the
    /// free list.
    pub fn peek_next_fresh(&self) -> TId {
        TId::from_usize_id(self.live_index_plus_one.end())
    }

    pub fn release(&mut self, id: TId) {
        let live_index_plus_one = self.live_index_plus_one[id.to_usize_id()];

        // Mark the id as no longer retained and push it onto the free list
        // for future recycling.
        self.live_index_plus_one[id.to_usize_id()] = 0;
        self.free.push(id);

        // Swap-remove: move the last live id into the slot vacated by the
        // released id to keep `live` packed.
        let last_id = self.live.pop().expect("released an id from an empty pool");
        if id == last_id {
            return;
        }

        self.live_index_plus_one[last_id.to_usize_id()] = live_index_plus_one;
        self.live[live_index_plus_one - 1] = last_id;
    }

    pub fn retain(&mut self) -> TId {
        // Recycle a freed id if one is available; otherwise allocate a
        // brand-new one (which also grows the reverse-index list).
        let id = match self.free.pop() {
            Some(free_id) => free_id,
            None => TId::from_usize_id(self.live_index_plus_one.push(0)),
        };

        self.live.push(id);
        // Store index + 1 so that 0 can mean "not retained".
        self.live_index_plus_one[id.to_usize_id()] = self.live.len();

        id
    }
}

impl<TId: Id> Default for IdStruct<TId> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, TId: Id> IntoIterator for &'a IdStruct<TId> {
    type Item = TId;
    type IntoIter = IdStructIter<'a, TId>;

    fn into_iter(self) -> Self::IntoIter {
        IdStructIter::from_live(&self.live)
    }
}
