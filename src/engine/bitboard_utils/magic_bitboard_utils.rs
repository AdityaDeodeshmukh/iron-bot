use crate::engine::bitboard_utils::base_operations::{get_bit, get_bit_nums, get_lsb_index, pop_bit, print_bit_board};

use super::base_operations::set_bit;

pub fn set_occupancy(index:u64,bits_in_mask:u8,mut attack_mask:u64) -> u64 {
    let mut occupancy_mask:u64 = 0;
    let mut sqr;
    for count in 0..bits_in_mask {
        unsafe {
            sqr = get_lsb_index(attack_mask);
        }
        attack_mask = pop_bit!(attack_mask,sqr);
        if (index & (1<<count)) != 0{
            occupancy_mask = set_bit!(occupancy_mask,sqr);
        }
    }
    occupancy_mask
}