use std::vec;

use crate::{engine::attack_maps, utils::util_enums::PlayerColor};
use super::{attack_maps::{  generate_bishop_attack_map, generate_king_attack_map, generate_knight_attack_map, generate_pawn_attack_map, generate_rook_attack_map, get_relevant_occupancy_bits_bishop, get_relevant_occupancy_bits_rook}, 
            bitboard_utils::{base_operations::print_bit_board, 
                bitboard_constants::{BISHOP_MAGIC_NUMBERS, NUM_OCCUPANCY_SQR_BISHOP, NUM_OCCUPANCY_SQR_ROOK, ROOK_MAGIC_NUMBERS}, 
                magic_bitboard_utils::set_occupancy}};

/*
Attack Maps implementation Details:
Pawn attack maps : 
pawn_attack_maps[0] == White attack maps
pawn_attack_maps[1] == Black attack maps
knight_attack_maps:
All attack maps for knight indexed by the square
king_attack_maps:
All attack maps for king indexed by the square
rook_attack_maps:
All attack maps for rook indexed by the rook magic numbers
bishop_attack_maps:
All attack maps for bisops indexed by the bishop magic numbers
bishop_relevant_occupancy:
All relevant occupancies for bishop
rook_relevant_occupancy:
All relevant occupancies for rook
 */
pub struct attack_map {
    pub pawn_attack_maps: Vec<Vec<u64>>,
    pub knight_attack_maps:Vec<u64>,
    pub king_attack_maps:Vec<u64>,
    pub rook_relevant_occupancy:Vec<u64>,
    pub bishop_relevant_occupancy:Vec<u64>,
    pub bishop_attack_maps:Vec<Vec<u64>>,
    pub rook_attack_maps:Vec<Vec<u64>>
}

impl attack_map {
    pub fn new() -> attack_map{
        let pawn_map;
        let king_map;
        let knight_map;
        let bishop_relevant_occupancy_map;
        let bishop_map;
        let rook_relevant_occupancy_map;
        let rook_map;
        (pawn_map,knight_map,king_map) = init_simple_pieces();
        (bishop_map,bishop_relevant_occupancy_map) = init_bishop();
        (rook_map,rook_relevant_occupancy_map) = init_rook();
        attack_map {
            pawn_attack_maps:pawn_map,
            knight_attack_maps:knight_map,
            king_attack_maps:king_map,
            rook_relevant_occupancy:rook_relevant_occupancy_map,
            bishop_relevant_occupancy:bishop_relevant_occupancy_map,
            bishop_attack_maps:bishop_map,
            rook_attack_maps:rook_map
        }
    }
}





//initialize the attack map for bishops with magic indices
pub fn init_bishop() -> (Vec<Vec<u64>>,Vec<u64>) {
    let mut attack_map:Vec<Vec<u64>> = vec![vec![0;512];64];
    let mut bishop_attacks:Vec<u64> = vec![0;64];
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
pub fn init_rook() -> (Vec<Vec<u64>>,Vec<u64>) {
    let mut attack_map:Vec<Vec<u64>> = vec![vec![0;4096];64];
    let mut rook_attacks:Vec<u64> = vec![0;64];
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
pub fn init_simple_pieces() -> (Vec<Vec<u64>>,Vec<u64>,Vec<u64>) {
    let mut pawn_attack_map:Vec<Vec<u64>> = vec![vec![0;64];2];
    let mut knight_attack_map:Vec<u64> = vec![0;64];
    let mut king_attack_map:Vec<u64> = vec![0;64];
    for square in 0..64 {
        pawn_attack_map[0][square as usize] = generate_pawn_attack_map(square, PlayerColor::White);
        pawn_attack_map[1][square as usize] = generate_pawn_attack_map(square, PlayerColor::Black);
        knight_attack_map[square as usize] = generate_knight_attack_map(square);
        king_attack_map[square as usize] = generate_king_attack_map(square);
    }
    (pawn_attack_map,knight_attack_map,king_attack_map)
}