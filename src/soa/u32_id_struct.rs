use crate::{ext::U32Ext, U32Id};
use std::{default::Default, mem::size_of};

pub struct U32IdStruct<TMarker> {
    used_ids: Vec<u64>,
    free_ids: Vec<U32Id<TMarker>>,
    next_id: U32Id<TMarker>,
}

impl<TMarker> U32IdStruct<TMarker> {
    pub fn new() -> Self {
        Self {
            used_ids: Vec::new(),
            free_ids: Vec::new(),
            next_id: 0u32.to_u32_id(),
        }
    }

    pub fn retain(&mut self) -> U32Id<TMarker> {
        let result = if let Some(free_id) = self.free_ids.pop() {
            // If we have a free id, use it
            free_id
        } else {
            // Otherwise, get the next id
            let result = self.next_id;
            self.next_id = (self.next_id.to_u32() + 1).to_u32_id();
            result
        };

        // Set the bit associated with the id
        set_bit(&mut self.used_ids, result.to_u32() as usize);

        result
    }

    pub fn release(&mut self, id: U32Id<TMarker>) {
        clear_bit(&mut self.used_ids, id.to_u32() as usize);
        self.free_ids.push(id);
    }
}

struct BitAccessInfo {
    slice_index: usize,
    u64_pattern: u64,
}

fn get_bit_access_info(index: usize) -> BitAccessInfo {
    let elem_size = size_of::<u64>();
    let used_ids_list_index = index / elem_size;
    let used_ids_bit_index = index % elem_size;
    let bit_pattern = 1 << used_ids_bit_index;
    BitAccessInfo {
        slice_index: used_ids_list_index,
        u64_pattern: bit_pattern,
    }
}

fn clear_bit(used_ids: &mut [u64], index: usize) {
    // 00000001 <- 0
    // 00000010 <- 1
    // 00000100 <- 2
    // 00001000 <- 3
    // 00010000 <- 4
    // 00100000 <- 5
    // 01000000 <- 6
    // 10000000 <- 7
    let bit_access_info = get_bit_access_info(index);

    let old_pattern = used_ids[bit_access_info.slice_index];
    let new_pattern = old_pattern & (!bit_access_info.u64_pattern);
    used_ids[bit_access_info.slice_index] = new_pattern;
}

fn set_bit(used_ids: &mut [u64], index: usize) {
    let bit_access_info = get_bit_access_info(index);

    let old_pattern = used_ids[bit_access_info.slice_index];
    let new_pattern = old_pattern | bit_access_info.u64_pattern;
    used_ids[bit_access_info.slice_index] = new_pattern;
}

impl<TMarker> Default for U32IdStruct<TMarker> {
    fn default() -> Self {
        Self::new()
    }
}
