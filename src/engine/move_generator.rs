use crate::utils::util_enums::PlayerColor;

use super::{game_state::game_state, init::attack_map, move_utils::{get_attacks_bishop,get_attacks_rook,get_attacks_queen}};

//a function to check if a particular square is being attacked by a side
#[inline(always)]
pub fn is_square_attacked(square:u8,side:&PlayerColor,game:&game_state,piece_attack_maps:&attack_map) -> i8 {
    match side{
        //check for white attacks
        PlayerColor::White => {
            //check for pawns
            if (piece_attack_maps.pawn_attack_maps[1][square as usize] 
                & game.piece_bitboards[0] & game.occupancy_bitboards[0])!=0 {return 1}
            
            //check for knights
            if (piece_attack_maps.knight_attack_maps[square as usize] &
                game.piece_bitboards[2] & game.occupancy_bitboards[0])!=0 {return 1}

            //check for king
            if (piece_attack_maps.king_attack_maps[square as usize] &
                game.piece_bitboards[5] & game.occupancy_bitboards[0])!=0 {return 1}
            
            //check for bishop
            let attacks_bishop = get_attacks_bishop(square, game.occupancy_bitboards[0] | game.occupancy_bitboards[1], 
                piece_attack_maps.bishop_relevant_occupancy, piece_attack_maps.bishop_attack_maps);
            if (attacks_bishop & game.piece_bitboards[3] & game.occupancy_bitboards[0])!=0 {return 1}
            
            //check for rook
            let attacks_rook = get_attacks_rook(square, game.occupancy_bitboards[0] | game.occupancy_bitboards[1], 
                piece_attack_maps.rook_relevant_occupancy, piece_attack_maps.rook_attack_maps);
            if (attacks_rook & game.piece_bitboards[1] & game.occupancy_bitboards[0])!=0 {return 1}
            
            //check for queen
            let attacks_queen = attacks_bishop | attacks_rook;
            if (attacks_queen & game.piece_bitboards[4] & game.occupancy_bitboards[0])!=0 {return 1}

            
        }

        //check for black attacks
        PlayerColor::Black => {
            //check for pawns
            if (piece_attack_maps.pawn_attack_maps[0][square as usize] 
                & game.piece_bitboards[0] & game.occupancy_bitboards[1])!=0 {return 1}

            //check for knights
            if (piece_attack_maps.knight_attack_maps[square as usize] &
                game.piece_bitboards[2] & game.occupancy_bitboards[1])!=0 {return 1}
            
            //check for king
            if (piece_attack_maps.king_attack_maps[square as usize] &
                game.piece_bitboards[5] & game.occupancy_bitboards[1])!=0 {return 1}

            //check for bishop
            let attacks_bishop = get_attacks_bishop(square, game.occupancy_bitboards[0] | game.occupancy_bitboards[1], 
                piece_attack_maps.bishop_relevant_occupancy, piece_attack_maps.bishop_attack_maps);
            if (attacks_bishop & game.piece_bitboards[3] & game.occupancy_bitboards[1])!=0 {return 1}
            
            //check for rook
            let attacks_rook = get_attacks_rook(square, game.occupancy_bitboards[0] | game.occupancy_bitboards[1], 
                piece_attack_maps.rook_relevant_occupancy, piece_attack_maps.rook_attack_maps);
            if (attacks_rook & game.piece_bitboards[1] & game.occupancy_bitboards[1])!=0 {return 1}

            //check for queen
            let attacks_queen = attacks_bishop | attacks_rook;
            if (attacks_queen & game.piece_bitboards[4] & game.occupancy_bitboards[1])!=0 {return 1}
        }
    }
    return 0
}

pub fn print_attacked_squares(side:&PlayerColor,game:&game_state,piece_attack_map:&attack_map) {
    for row in (0..=7).rev() {
        print!("{}   ",row+1);
        for file in 0..=7 {
            let square = row*8 + file;
            print!("{} ",is_square_attacked(square, &side, &game, &piece_attack_map));
        }
        println!("");
    }
    println!("");
    println!("    a b c d e f g h");
    println!("Side:{}",if matches!(side,PlayerColor::White){"White"}else{"Black"});
}