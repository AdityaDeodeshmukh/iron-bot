use core::num;

use crate::engine::bitboard_utils::bitboard_constants::{NOT_A_FILE,NOT_H_FILE,NOT_AB_FILE,
                                                        NOT_12_ROW,NOT_1_ROW,NOT_78_ROW,
                                                        NOT_8_ROW,NOT_GH_FILE
                                                        };
use crate::utils::util_enums::PlayerColor;
use crate::engine::bitboard_utils::base_operations::set_bit;
use crate::engine::bitboard_utils::bitboard_constants::{NUM_OCCUPANCY_SQR_BISHOP,NUM_OCCUPANCY_SQR_ROOK};
use super::bitboard_utils::base_operations::{get_bit, get_bit_nums, print_bit_board};
use super::bitboard_utils::magic_bitboard_utils::set_occupancy;
use super::bitboard_utils::rng_utils::generate_magic_number_candidate;
use super::init::{init_bishop, init_simple_pieces};



//generate a map that stores the squares a pawn can attack when placed on a certain position
pub fn generate_pawn_attack_map(square:u8,side:PlayerColor) -> u64 {
    let mut bitboard = 0;
    bitboard = set_bit!(bitboard,square);
    match side{
        PlayerColor::Black => {
            return (bitboard&NOT_A_FILE)>>9 | (bitboard&NOT_H_FILE)>>7
        }
        PlayerColor::White => {
            return (bitboard&NOT_A_FILE)<<7 | (bitboard&NOT_H_FILE)<<9
        }
    } 
}

//generate a map that stores the squares a knight can attack when placed on a certain position
pub fn generate_knight_attack_map(square:u8) -> u64 {
    let mut attack_map = 0;
    let mut bitboard:u64 = 0;
    bitboard = set_bit!(bitboard,square);
    if (bitboard&NOT_78_ROW&NOT_A_FILE) !=0 {
        attack_map |= bitboard << 15; 
    }
    if (bitboard&NOT_78_ROW&NOT_H_FILE) !=0 {
        attack_map |= bitboard << 17; 
    }
    if (bitboard&NOT_8_ROW&NOT_AB_FILE) !=0 {
        attack_map |= bitboard <<6;
    }
    if (bitboard&NOT_8_ROW&NOT_GH_FILE) !=0 {
        attack_map |= bitboard <<10;
    }
    if (bitboard&NOT_1_ROW&NOT_AB_FILE) !=0 {
        attack_map |= bitboard >> 10;
    }
    if (bitboard&NOT_1_ROW&NOT_GH_FILE) !=0 {
        attack_map |= bitboard >> 6;
    }
    if (bitboard&NOT_12_ROW&NOT_H_FILE) !=0 {
        attack_map |= bitboard >> 15; 
    }
    if (bitboard&NOT_12_ROW&NOT_A_FILE) !=0 {
        attack_map |= bitboard >> 17; 
    }
    attack_map
}

//generate a map that stores the squares a king can attack when placed on a certain position
pub fn generate_king_attack_map(square:u8) -> u64{
    let mut attack_map = 0;
    let mut bitboard:u64 = 0;
    bitboard = set_bit!(bitboard,square);
    if (bitboard&NOT_8_ROW) != 0{
        attack_map |= bitboard << 8;
    }
    if (bitboard&NOT_1_ROW) != 0{
        attack_map |= bitboard >> 8;
    }
    if (bitboard&NOT_A_FILE) != 0{
        attack_map |= bitboard >> 1;
    }
    if (bitboard&NOT_H_FILE) != 0{
        attack_map |= bitboard << 1;
    }
    if (bitboard&NOT_A_FILE&NOT_1_ROW) !=0 {
        attack_map |= bitboard >> 9;
    }
    if (bitboard&NOT_H_FILE&NOT_1_ROW) !=0 {
        attack_map |= bitboard >> 7;
    }
    if (bitboard&NOT_A_FILE&NOT_8_ROW) !=0 {
        attack_map |= bitboard << 7;
    }
    if (bitboard&NOT_H_FILE&NOT_8_ROW) !=0 {
        attack_map |= bitboard << 9;
    }
    attack_map
}

//generate a map storing the relevant occupancy squares for a bishop on a particular square
pub fn get_relevant_occupancy_bits_bishop(square:u8) -> u64{
    let mut attacks:u64 = 0;
    let blank_bb:u64 = 0;
    let tar_f = square%8;
    let tar_r = square/8;

    //travel top right
    for (r,f) in ((tar_r+1)..=6).zip((tar_f+1)..=6) {
        let sqr = r*8+f;
        attacks |= set_bit!(blank_bb,sqr);
    }

    //travel top left
    if tar_f > 0 {
        for (r,f) in ((tar_r+1)..=6).zip((1..=(tar_f-1)).rev()) {
            let sqr = r*8+f;
            attacks |= set_bit!(blank_bb,sqr);
        }
    }
    
    //travel bottom right
    if tar_r > 0 {
        for (r,f) in ((1..=(tar_r-1)).rev()).zip((tar_f+1)..=6) {
            let sqr = r*8+f;
            attacks |= set_bit!(blank_bb,sqr);
        }
    }
    
    //travel bottom left
    if tar_r > 0 && tar_f > 0 {
        for (r,f) in ((1..=(tar_r-1)).rev()).zip((1..=(tar_f-1)).rev()) {
            let sqr = r*8+f;
            attacks |= set_bit!(blank_bb,sqr);
        }
    }
    
    attacks
}

//generate a map storing the relevant occupancy squares for a rook on a particular square
pub fn get_relevant_occupancy_bits_rook(square:u8) -> u64 {
    let mut attacks:u64 = 0;
    let blank_bb:u64 = 0;
    let tar_f = square%8;
    let tar_r = square/8;

    //travel down
    if tar_r > 0 {
        for r in 1..=(tar_r-1) {
            let sqr = r*8 + tar_f;
            attacks |= set_bit!(blank_bb,sqr);
        }
    }
    
    //travel up
    for r in (tar_r+1)..=6 {
        let sqr = r*8 + tar_f;
        attacks |= set_bit!(blank_bb,sqr);
    }

    //travel left
    if tar_f > 0 {
        for f in 1..=(tar_f-1) {
            let sqr = tar_r*8 + f;
            attacks |= set_bit!(blank_bb,sqr);
        }
    }
    
    //travel right
    for f in (tar_f+1)..=6 {
        let sqr = tar_r*8 + f;
        attacks |= set_bit!(blank_bb,sqr);
    }
    attacks
}

//generate a bitmap storing rook attacks based on where other pieces are
pub fn generate_rook_attack_map(square:u8,block_map:u64) -> u64 {
    let mut attacks:u64 = 0;
    let blank_bb:u64 = 0;
    let tar_f = square%8;
    let tar_r = square/8;

    //travel down
    if tar_r > 0 {
        for r in (0..=(tar_r-1)).rev() {
            let sqr = r*8 + tar_f;
            let pos = set_bit!(blank_bb,sqr);
            attacks |= pos;
            if pos & block_map != 0 {
                break;
            }
        }
    }
    
    //travel up
    for r in (tar_r+1)..=7 {
        let sqr = r*8 + tar_f;
        let pos = set_bit!(blank_bb,sqr);
        attacks |= pos;
        if pos & block_map != 0 {
            break;
        }
    }

    //travel left
    if tar_f > 0 {
        for f in (0..=(tar_f-1)).rev() {
            let sqr = tar_r*8 + f;
            let pos = set_bit!(blank_bb,sqr);
            attacks |= pos;
            if pos & block_map != 0 {
                break;
            }
        }
    }
    
    //travel right
    for f in (tar_f+1)..=7 {
        let sqr = tar_r*8 + f;
        let pos = set_bit!(blank_bb,sqr);
        attacks |= pos;
        if pos & block_map != 0 {
            break;
        }
    }
    attacks
}

//generate a bitmap storing bishop attacks based on where other pieces are
pub fn generate_bishop_attack_map(square:u8,block_map:u64) -> u64{
    let mut attacks:u64 = 0;
    let blank_bb:u64 = 0;
    let tar_f = square%8;
    let tar_r = square/8;

    //travel top right
    for (r,f) in ((tar_r+1)..=7).zip((tar_f+1)..=7) {
        let sqr = r*8+f;
        attacks |= set_bit!(blank_bb,sqr);
        if get_bit!(block_map,sqr) == 1 {
            break;
        }
    }

    //travel top left
    if tar_f > 0 {
        for (r,f) in ((tar_r+1)..=7).zip((0..=(tar_f-1)).rev()) {
            let sqr = r*8+f;
            attacks |= set_bit!(blank_bb,sqr);
            if get_bit!(block_map,sqr) == 1 {
                break;
            }
        }
    }
    
    //travel bottom right
    if tar_r > 0 {
        for (r,f) in ((0..=(tar_r-1)).rev()).zip((tar_f+1)..=7) {
            let sqr = r*8+f;
            attacks |= set_bit!(blank_bb,sqr);
            if get_bit!(block_map,sqr) == 1 {
                break;
            }
        }
    }

    //travel bottom left
    if tar_r > 0 && tar_f > 0 {
        for (r,f) in ((0..=(tar_r-1)).rev()).zip((0..=(tar_f-1)).rev()) {
            let sqr = r*8+f;
            attacks |= set_bit!(blank_bb,sqr);
            if get_bit!(block_map,sqr) == 1 {
                break;
            }
        }
    }
    attacks
}

//generate magic number for bishop on a square
pub fn generate_magic_bitboard_bishop(square:u8,mut seed:u32) -> (u64,u32){
    let mut attacks:[u64;512] = [0;512];
    let mut used_attacks:[u64;512] = [0;512];
    let mut occupancies:[u64;512] = [0;512];
    let attack_map = get_relevant_occupancy_bits_bishop(square);
    let relevant_bits = NUM_OCCUPANCY_SQR_BISHOP[square as usize];
    let occupancy_indices = 1 << relevant_bits;
    for index in 0..occupancy_indices {
        //generate all possible relevant occupancies
        occupancies[index] = set_occupancy(index as u64, relevant_bits, attack_map);
        //generate attacks for said occupancies
        attacks[index] = generate_bishop_attack_map(square, occupancies[index]);
    }
    for cnt in 0..1000000 {
        let magic_candidate;
        //generate magic number candidate
        (magic_candidate,seed)= generate_magic_number_candidate(seed);

        //early filtering
        if unsafe{get_bit_nums((attack_map.wrapping_mul(magic_candidate)) & 0xFF00000000000000) < 6} {
            continue;
        }

        used_attacks = [0;512];
        let mut fail = false;

        //go through each type of occupancies possible
        for index in 0..occupancy_indices {
            //get the magic index
            let magic_index = ((occupancies[index].wrapping_mul(magic_candidate)) >> (64 - relevant_bits)) as usize;

            //if new magic index is found
            if used_attacks[magic_index] == 0 {
                used_attacks[magic_index] = attacks[index];
            }

            //if magic number fails (Already a different attack pattern exists at the magic index)
            else if used_attacks[magic_index] != attacks[index] {
                fail = true;
                break;
            }
        }
        if !fail {
            return (magic_candidate,seed);
        }
    }
    // Oh no! :(
    println!("Magic number not found");
    (0,seed)
}

//generate magic number for rook on a square
pub fn generate_magic_bitboard_rook(square:u8,mut seed:u32) -> (u64,u32){
    let mut attacks:[u64;4096] = [0;4096];
    let mut used_attacks:[u64;4096] = [0;4096];
    let mut occupancies:[u64;4096] = [0;4096];
    let attack_map = get_relevant_occupancy_bits_rook(square);
    let relevant_bits = NUM_OCCUPANCY_SQR_ROOK[square as usize];
    let occupancy_indices = 1 << relevant_bits;
    for index in 0..occupancy_indices {
        //generate all possible relevant occupancies
        occupancies[index] = set_occupancy(index as u64, relevant_bits, attack_map);
        //generate attacks for said occupancies
        attacks[index] = generate_rook_attack_map(square, occupancies[index]);
    }
    for cnt in 0..100000000 {
        let magic_candidate;
        //generate magic number candidate
        (magic_candidate,seed)= generate_magic_number_candidate(seed);

        //early filtering
        if unsafe{get_bit_nums((attack_map.wrapping_mul(magic_candidate)) & 0xFF00000000000000) < 6} {
            continue;
        }

        used_attacks = [0;4096];
        let mut fail = false;

        //go through each type of occupancies possible
        for index in 0..occupancy_indices {
            //get the magic index
            let magic_index = ((occupancies[index].wrapping_mul(magic_candidate)) >> (64 - relevant_bits)) as usize;

            //if new magic index is found
            if used_attacks[magic_index] == 0 {
                used_attacks[magic_index] = attacks[index];
            }

            //if magic number fails (Already a different attack pattern exists at the magic index)
            else if used_attacks[magic_index] != attacks[index] {
                fail = true;
                break;
            }
        }

        if !fail {
            return (magic_candidate,seed);
        }
    }
    // Oh no! :(
    println!("Magic number not found");
    (0,seed)
}