use crate::{ext::UsizeExt, soa::BitAccessInfo, UsizeId};
use std::marker::PhantomData;

pub struct UsizeIdStructIter<'a, TMarker: ?Sized> {
    used_ids: &'a Vec<u64>,
    idx: usize,
    phantom: PhantomData<TMarker>,
}

impl<'a, TMarker: ?Sized> UsizeIdStructIter<'a, TMarker> {
    pub fn from_used_ids(used_ids: &'a Vec<u64>) -> Self {
        Self {
            used_ids,
            idx: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a, TMarker: ?Sized> Iterator for UsizeIdStructIter<'a, TMarker> {
    type Item = UsizeId<TMarker>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut bit_access_info = BitAccessInfo::from_index(self.idx);

        while bit_access_info.slice_index < self.used_ids.len() {
            // Advance the index
            self.idx += 1;

            if self.used_ids[bit_access_info.slice_index] & bit_access_info.u64_pattern != 0 {
                return Some((self.idx - 1).to_usize_id());
            }

            // Get the next `BitAccessInfo`.
            bit_access_info = BitAccessInfo::from_index(self.idx);
        }

        None
    }
}
