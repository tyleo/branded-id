use std::mem::size_of;

pub struct BitAccessInfo {
    pub slice_index: usize,
    pub u64_pattern: u64,
}

impl BitAccessInfo {
    pub fn from_index(index: usize) -> BitAccessInfo {
        // 00000001 <- 0
        // 00000010 <- 1
        // 00000100 <- 2
        // 00001000 <- 3
        // 00010000 <- 4
        // 00100000 <- 5
        // 01000000 <- 6
        // 10000000 <- 7
        let elem_size = size_of::<u64>();
        let used_ids_list_index = index / elem_size;
        let used_ids_bit_index = index % elem_size;
        let bit_pattern = 1 << used_ids_bit_index;
        BitAccessInfo {
            slice_index: used_ids_list_index,
            u64_pattern: bit_pattern,
        }
    }
}
