use crate::utils::util_enums::PlayerColor;

use crate::engine::bitboard_utils::bitboard_constants::{BISHOP_MAGIC_NUMBERS, 
                                                        NUM_OCCUPANCY_SQR_BISHOP, NUM_OCCUPANCY_SQR_ROOK, ROOK_MAGIC_NUMBERS};
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

//get queen attacks from the attack map based on the square and occupancy
#[inline(always)]
pub fn get_attacks_queen(square:u8,occupancy:u64,rook_masks:[u64;64],rook_attacks:[[u64;4096];64],
                         bishop_masks:[u64;64],bishop_attacks:[[u64;512];64]) -> u64{
    //get the rook map
    let mut occ_rook = occupancy & rook_masks[square as usize];
    occ_rook = occ_rook.wrapping_mul(ROOK_MAGIC_NUMBERS[square as usize]);
    occ_rook = occ_rook >> (64-NUM_OCCUPANCY_SQR_ROOK[square as usize]);
    let rook_map = rook_attacks[square as usize][occ_rook as usize];
    //get the bishop map
    let mut occ_bishop = occupancy & bishop_masks[square as usize];
    occ_bishop = occ_bishop.wrapping_mul(BISHOP_MAGIC_NUMBERS[square as usize]);
    occ_bishop = occ_bishop >> (64-NUM_OCCUPANCY_SQR_BISHOP[square as usize]);
    let bishop_map = bishop_attacks[square as usize][occ_bishop as usize];
    //queen map = (rook map) or (bishop map)
    rook_map|bishop_map
}

//function to check if a given move is legal
pub fn is_move_legal(_initial_idx:u32,_final_idx:u32) -> bool{
    true
}
//function to change the board state once a move is played
pub fn change_board(game_state: &mut ([i32; 64], u32, PlayerColor, bool, bool, bool, bool, u32, u32),
                    initial_idx:u32,
                    final_idx:u32) {
                        let piece = game_state.0[initial_idx as usize];
                        game_state.0[initial_idx as usize] = 0;
                        game_state.0[final_idx as usize] = piece;
}