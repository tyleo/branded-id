use crate::{ext::UsizeExt, UsizeId};
use std::default::Default;

use super::{BitAccessInfo, UsizeIdStructIter};

pub struct UsizeIdStruct<TMarker: ?Sized> {
    used_ids: Vec<u64>,
    free_ids: Vec<UsizeId<TMarker>>,
    next_id: UsizeId<TMarker>,
}

impl<TMarker> UsizeIdStruct<TMarker> {
    pub fn new() -> Self {
        Self {
            used_ids: Vec::new(),
            free_ids: Vec::new(),
            next_id: 0usize.to_usize_id(),
        }
    }

    pub fn count(&self) -> usize {
        self.next_id.to_usize() - self.free_ids.len()
    }

    pub fn retain(&mut self) -> UsizeId<TMarker> {
        let result = if let Some(free_id) = self.free_ids.pop() {
            // If we have a free id, use it
            free_id
        } else {
            // Otherwise, get the next id
            let result = self.next_id;
            self.next_id = (self.next_id.to_usize() + 1).to_usize_id();
            result
        };

        // Set the bit associated with the id
        set_bit(&mut self.used_ids, result.to_usize());

        result
    }

    pub fn release(&mut self, id: UsizeId<TMarker>) {
        clear_bit(&mut self.used_ids, id.to_usize());
        self.free_ids.push(id);
    }
}

fn clear_bit(used_ids: &mut [u64], index: usize) {
    let bit_access_info = BitAccessInfo::from_index(index);

    // Clear the bit
    let old_pattern = used_ids[bit_access_info.slice_index];
    let new_pattern = old_pattern & (!bit_access_info.u64_pattern);
    used_ids[bit_access_info.slice_index] = new_pattern;
}

fn set_bit(used_ids: &mut Vec<u64>, index: usize) {
    let bit_access_info = BitAccessInfo::from_index(index);

    // Ensure the bit we need to set actually exists
    ensure_size(used_ids, bit_access_info.slice_index + 1);

    // Set the bit
    let old_pattern = used_ids[bit_access_info.slice_index];
    let new_pattern = old_pattern | bit_access_info.u64_pattern;
    used_ids[bit_access_info.slice_index] = new_pattern;
}

fn ensure_size(items: &mut Vec<u64>, desired_size: usize) {
    while items.len() < desired_size {
        items.push(0);
    }
}

impl<TMarker> Default for UsizeIdStruct<TMarker> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, TMarker> IntoIterator for &'a UsizeIdStruct<TMarker> {
    type Item = UsizeId<TMarker>;
    type IntoIter = UsizeIdStructIter<'a, TMarker>;

    fn into_iter(self) -> Self::IntoIter {
        UsizeIdStructIter::from_used_ids(&self.used_ids)
    }
}
