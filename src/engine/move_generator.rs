
use crate::{engine::bitboard_utils::base_operations::pop_bit, utils::util_enums::{PlayerColor}};

use super::{bitboard_utils::{base_operations::{get_lsb_index, print_bit_board}, bitboard_constants::{IS_23456_ROW, IS_2_ROW, IS_34567_ROW, IS_7_ROW}}, game_state::game_state, init::attack_map, move_utils::{get_attacks_bishop, get_attacks_queen, get_attacks_rook}};

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

//a function to print which squares are being attacked by a particular side
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



pub fn generate_moves(side:&PlayerColor,game:&game_state,piece_attack_map:&attack_map) {
    let mut starting_square:u8 ;
    let mut ending_square:u8 ;
    let mut single_pawn_moves:u64;
    let all_occupancies = game.occupancy_bitboards[0] | game.occupancy_bitboards[1];
    let side_pawn_bitboard;
    // Capturing pawn moves
    match side {
        PlayerColor::White => {
            side_pawn_bitboard = game.piece_bitboards[0] & game.occupancy_bitboards[0];
        }
        PlayerColor::Black => {
            side_pawn_bitboard = game.piece_bitboards[0] & game.occupancy_bitboards[1];
        }
    }

    //Handling Single Pawn moves
    match side {
        PlayerColor::White => {
            single_pawn_moves = (side_pawn_bitboard & IS_23456_ROW) & (!all_occupancies>>8);
        }
        PlayerColor::Black => {
            single_pawn_moves = (side_pawn_bitboard & IS_34567_ROW) & (!all_occupancies<<8);
        }
    }
    
    let single_pawn_moves_cpy = single_pawn_moves;
    while single_pawn_moves != 0 {
        unsafe{
            starting_square = get_lsb_index(single_pawn_moves);
        }
        match side{
            PlayerColor::White => {
                ending_square = starting_square + 8;
            }
            PlayerColor::Black => {
                ending_square = starting_square - 8;
            }
        }
        println!("Pawn move from {}{} to {}{}",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
        single_pawn_moves= pop_bit!(single_pawn_moves,starting_square);
    }

    //Handling double pawn moves
    let mut double_pawn_moves;
    match side {
        PlayerColor::White => {
            double_pawn_moves = (side_pawn_bitboard & IS_2_ROW) & 
                                (!all_occupancies>>16) & single_pawn_moves_cpy;

        }
        PlayerColor::Black => {
            double_pawn_moves = (side_pawn_bitboard & IS_7_ROW) & 
                                    (!all_occupancies<<16) & single_pawn_moves_cpy;
        }
    }
    while double_pawn_moves != 0 {
        unsafe{
            starting_square = get_lsb_index(double_pawn_moves);
        }
        match side{
            PlayerColor::White => {
                ending_square = starting_square + 16;
            }
            PlayerColor::Black => {
                ending_square = starting_square - 16;
            }
        }
        println!("Pawn move from {}{} to {}{}",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
        double_pawn_moves= pop_bit!(double_pawn_moves,starting_square);
    }

    //Handling Pawn Promotions
    let mut pawn_promotion_moves:u64;
    match side {
        PlayerColor::White => {
            pawn_promotion_moves = (side_pawn_bitboard & IS_7_ROW) & (!all_occupancies>>8);
        }
        PlayerColor::Black => {
            pawn_promotion_moves = (side_pawn_bitboard & IS_2_ROW) & (!all_occupancies<<8);
        }
    }
    while pawn_promotion_moves != 0 {
        unsafe{
            starting_square = get_lsb_index(pawn_promotion_moves);
        }
        match side{
            PlayerColor::White => {
                ending_square = starting_square + 8;
            }
            PlayerColor::Black => {
                ending_square = starting_square - 8;
            }
        }
        println!("Pawn promotion from {}{} to {}{} to Queen",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
        println!("Pawn promotion from {}{} to {}{} to Rook",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
        println!("Pawn promotion from {}{} to {}{} to Bishop",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
        println!("Pawn promotion from {}{} to {}{} to Knight",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));

        pawn_promotion_moves= pop_bit!(pawn_promotion_moves,starting_square);
    }

    //Handling Pawn Attacks
    let mut pawn_bitboard = side_pawn_bitboard;
    let mut pawn_attack_map;
    while pawn_bitboard !=0 {
        unsafe{
            starting_square = get_lsb_index(pawn_bitboard);
        }
        //getting the appropriate attack map
        match side{
            PlayerColor::White => {
                pawn_attack_map = piece_attack_map.pawn_attack_maps[0][starting_square as usize] & game.occupancy_bitboards[1];
            }
            PlayerColor::Black => {
                pawn_attack_map = piece_attack_map.pawn_attack_maps[1][starting_square as usize] & game.occupancy_bitboards[0];
            }
        }
        while pawn_attack_map !=0 {
            unsafe {
                ending_square = get_lsb_index(pawn_attack_map);
            }

            //handling pawn capture promotions
            match side{
                PlayerColor::White => {
                    if ending_square >= 56 {
                        println!("Pawn Capture Promotion from {}{} to {}{} to Queen",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
                        println!("Pawn Capture Promotion from {}{} to {}{} to Rook",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
                        println!("Pawn Capture Promotion from {}{} to {}{} to Bishop",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
                        println!("Pawn Capture Promotion from {}{} to {}{} to Knight",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
                        pawn_attack_map = pop_bit!(pawn_attack_map,ending_square);
                        continue;
                    }
                }
                PlayerColor::Black => {
                    if ending_square <= 7 {
                        println!("Pawn Capture Promotion from {}{} to {}{} to Queen",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
                        println!("Pawn Capture Promotion from {}{} to {}{} to Rook",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
                        println!("Pawn Capture Promotion from {}{} to {}{} to Bishop",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
                        println!("Pawn Capture Promotion from {}{} to {}{} to Knight",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
                        pawn_attack_map = pop_bit!(pawn_attack_map,ending_square);
                        continue;
                    }
                }
            }
            //handling normal pawn captures
            println!("Pawn Capture from {}{} to {}{}",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((ending_square%8+b'a') as char),((ending_square/8+b'1') as char));
            pawn_attack_map = pop_bit!(pawn_attack_map,ending_square);
        }
        pawn_bitboard = pop_bit!(pawn_bitboard,starting_square); 
    }

    //Handling en-pessant captures
    let en_pessant_square = game.en_pessant_square;
    let mut en_pessant_attackers;
    match en_pessant_square {
        0..=63 => {
                match side {
                    PlayerColor::White => {
                        en_pessant_attackers = piece_attack_map.pawn_attack_maps[1][en_pessant_square as usize] & side_pawn_bitboard;
                    }
                    PlayerColor::Black => {
                        en_pessant_attackers = piece_attack_map.pawn_attack_maps[0][en_pessant_square as usize] & side_pawn_bitboard;
                    }
                }
                while en_pessant_attackers != 0 {
                    unsafe {
                        starting_square = get_lsb_index(en_pessant_attackers);
                    }
                    println!("Pawn en pessant capture from {}{} to {}{}",((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),((en_pessant_square%8+b'a') as char),((en_pessant_square/8+b'1') as char));
                    en_pessant_attackers = pop_bit!(en_pessant_attackers,starting_square);
                }
            }
        
        65 => {
            ()
        }
        _ => {
            panic!("en pessant square cannot be that number{}",en_pessant_square);
        }
    }
    
    //Handling Castling Moves
    let king_position;
    match side {
        PlayerColor::White => {
            unsafe {
                king_position = get_lsb_index(game.piece_bitboards[5] & game.occupancy_bitboards[0]);
            }
        }
        PlayerColor::Black => {
            unsafe {
                king_position = get_lsb_index(game.piece_bitboards[5] & game.occupancy_bitboards[1]);
            }
        } 
    }




    

    
}