use crate::{engine::attack_maps, utils::util_enums::PlayerColor};
use super::{attack_maps::{  generate_bishop_attack_map, generate_king_attack_map, generate_knight_attack_map, generate_pawn_attack_map, generate_rook_attack_map, get_relevant_occupancy_bits_bishop, get_relevant_occupancy_bits_rook}, 
            bitboard_utils::{base_operations::print_bit_board, 
                bitboard_constants::{BISHOP_MAGIC_NUMBERS, NUM_OCCUPANCY_SQR_BISHOP, NUM_OCCUPANCY_SQR_ROOK, ROOK_MAGIC_NUMBERS}, 
                magic_bitboard_utils::set_occupancy}};


//get bishop attacks from the attack map based on the square and occupancy
#[inline(always)]
pub fn get_attacks_bishop(square:u8,occupancy:u64,bishop_masks:[u64;64],bishop_attacks:[[u64;512];64]) -> u64{
    let mut occ = occupancy & bishop_masks[square as usize];
    occ = occ.wrapping_mul(BISHOP_MAGIC_NUMBERS[square as usize]);
    occ = occ >> (64-NUM_OCCUPANCY_SQR_BISHOP[square as usize]);
    bishop_attacks[square as usize][occ as usize]
}

//get rook attacks from the attack map based on the square and occupancy
#[inline(always)]
pub fn get_attacks_rook(square:u8,occupancy:u64,rook_masks:[u64;64],rook_attacks:[[u64;4096];64]) -> u64{
    let mut occ = occupancy & rook_masks[square as usize];
    occ = occ.wrapping_mul(ROOK_MAGIC_NUMBERS[square as usize]);
    occ = occ >> (64-NUM_OCCUPANCY_SQR_ROOK[square as usize]);
    rook_attacks[square as usize][occ as usize]
}


//initialize the attack map for bishops with magic indices
pub fn init_bishop() -> ([[u64;512];64],[u64;64]) {
    let mut attack_map:[[u64;512];64] = [[0;512];64];
    let mut bishop_attacks:[u64;64] = [0;64];
    for square in 0..64 {
        let magic_number = BISHOP_MAGIC_NUMBERS[square as usize];

        //get the occupancy map
        bishop_attacks[square as usize] = get_relevant_occupancy_bits_bishop(square);
        let attack_mask = bishop_attacks[square as usize];
        let occupancy_indices = 1 << NUM_OCCUPANCY_SQR_BISHOP[square as usize];

        //go through each type of occupancies possible
        for index in 0..occupancy_indices {
            let occupancy = set_occupancy(index, NUM_OCCUPANCY_SQR_BISHOP[square as usize], attack_mask);
            let magic_index = occupancy.wrapping_mul(magic_number) >> (64 - NUM_OCCUPANCY_SQR_BISHOP[square as usize]);
            attack_map[square as usize][magic_index as usize] = generate_bishop_attack_map(square, occupancy);
        }
    }
    (attack_map,bishop_attacks)
}

//initialize the attack map for rooks with magic indices
pub fn init_rook() -> ([[u64;4096];64],[u64;64]) {
    let mut attack_map:[[u64;4096];64] = [[0;4096];64];
    let mut rook_attacks:[u64;64] = [0;64];
    for square in 0..64 {
        let magic_number = ROOK_MAGIC_NUMBERS[square as usize];

        //get the occupancy map
        rook_attacks[square as usize] = get_relevant_occupancy_bits_rook(square);
        let attack_mask = rook_attacks[square as usize];
        let occupancy_indices = 1 << NUM_OCCUPANCY_SQR_ROOK[square as usize];

        //go through each type of occupancies possible
        for index in 0..occupancy_indices {
            let occupancy = set_occupancy(index, NUM_OCCUPANCY_SQR_ROOK[square as usize], attack_mask);
            let magic_index = occupancy.wrapping_mul(magic_number) >> (64 - NUM_OCCUPANCY_SQR_ROOK[square as usize]);
            attack_map[square as usize][magic_index as usize] = generate_rook_attack_map(square, occupancy);
        }
    }
    (attack_map,rook_attacks)
}


//initialize attack maps for pawn,knight and king
pub fn init_simple_pieces() -> ([[u64;64];2],[u64;64],[u64;64]) {
    let mut pawn_attack_map:[[u64;64];2] = [[0;64];2];
    let mut knight_attack_map:[u64;64] = [0;64];
    let mut king_attack_map:[u64;64] = [0;64];
    for square in 0..64 {
        pawn_attack_map[0][square as usize] = generate_pawn_attack_map(square, PlayerColor::White);
        pawn_attack_map[1][square as usize] = generate_pawn_attack_map(square, PlayerColor::Black);
        knight_attack_map[square as usize] = generate_knight_attack_map(square);
        king_attack_map[square as usize] = generate_king_attack_map(square);
    }
    (pawn_attack_map,knight_attack_map,king_attack_map)
}