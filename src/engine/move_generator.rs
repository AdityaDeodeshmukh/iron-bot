
use crate::{engine::{binary_move::{encode_move, print_move}, bitboard_utils::base_operations::pop_bit}, utils::util_enums::PlayerColor};

use super::{binary_move::MoveList, bitboard_utils::{base_operations::{get_bit, get_lsb_index, print_bit_board}, bitboard_constants::{BLACK_KING_SIDE_CASTLE, BLACK_QUEEN_SIDE_CASTLE, FILLED_BOARD, IS_23456_ROW, IS_2_ROW, IS_34567_ROW, IS_7_ROW, PIECE_BISHOP, PIECE_KING, PIECE_KNIGHT, PIECE_NONE, PIECE_PAWN, PIECE_QUEEN, PIECE_ROOK, PROMOTION_BISHOP, PROMOTION_KNIGHT, PROMOTION_QUEEN, PROMOTION_ROOK, WHITE_KING_SIDE_CASTLE, WHITE_QUEEN_SIDE_CASTLE}}, game_state::game_state, init::attack_map, move_utils::{get_attacks_bishop, get_attacks_queen, get_attacks_rook}};

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
                &piece_attack_maps.bishop_relevant_occupancy, &piece_attack_maps.bishop_attack_maps);
            if (attacks_bishop & game.piece_bitboards[3] & game.occupancy_bitboards[0])!=0 {return 1}
            
            //check for rook
            let attacks_rook = get_attacks_rook(square, game.occupancy_bitboards[0] | game.occupancy_bitboards[1], 
                &piece_attack_maps.rook_relevant_occupancy, &piece_attack_maps.rook_attack_maps);
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
                &piece_attack_maps.bishop_relevant_occupancy, &piece_attack_maps.bishop_attack_maps);
            if (attacks_bishop & game.piece_bitboards[3] & game.occupancy_bitboards[1])!=0 {return 1}
            
            //check for rook
            let attacks_rook = get_attacks_rook(square, game.occupancy_bitboards[0] | game.occupancy_bitboards[1], 
                &piece_attack_maps.rook_relevant_occupancy, &piece_attack_maps.rook_attack_maps);
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

//a function to get whether a position is in check and the type of check (single, double, no check)
#[inline(always)]
pub fn get_check_type(side:PlayerColor,game:&game_state,piece_attack_map:&attack_map,king_position:u8,all_occupancies:u64) -> (u8,u64) {
    let mut defense_pattern:u64 = FILLED_BOARD;
    let mut check_type:u8 = 0;
    let mut attacker_map;
    let mut opponent_attacker_pattern:u64;
    let mut attacker_position:u8;
  

    match side {
        PlayerColor::White => {
            let bishop_attack_pattern = get_attacks_bishop(king_position, all_occupancies, 
                                                &piece_attack_map.bishop_relevant_occupancy, 
                                                &piece_attack_map.bishop_attack_maps);
            
            //handle diagonal attacks from bishops and queens
            attacker_map = bishop_attack_pattern & ((game.piece_bitboards[3] | game.piece_bitboards[4]) & game.occupancy_bitboards[1]);
            if attacker_map != 0 {
                unsafe {
                    attacker_position = get_lsb_index(attacker_map);
                }
                check_type = check_type + 1;
                opponent_attacker_pattern = get_attacks_bishop(attacker_position, all_occupancies, 
                                                        &piece_attack_map.bishop_relevant_occupancy, 
                                                        &piece_attack_map.bishop_attack_maps);
                defense_pattern = (bishop_attack_pattern & opponent_attacker_pattern) | attacker_map;
            }
            let rook_attack_pattern = get_attacks_rook(king_position, all_occupancies, 
                                                &piece_attack_map.rook_relevant_occupancy, 
                                                &piece_attack_map.rook_attack_maps);
            


            //handle straight attacks from rooks and queens
            attacker_map = rook_attack_pattern & ((game.piece_bitboards[1] | game.piece_bitboards[4]) & game.occupancy_bitboards[1]);
            if attacker_map != 0 {
                unsafe {
                    attacker_position = get_lsb_index(attacker_map);
                }
                check_type = check_type + 1;
                if check_type >= 2{return (2,0)}
                opponent_attacker_pattern = get_attacks_rook(attacker_position, all_occupancies, 
                                                        &piece_attack_map.rook_relevant_occupancy, 
                                                        &piece_attack_map.rook_attack_maps);
                defense_pattern = (rook_attack_pattern & opponent_attacker_pattern) | attacker_map;
            }

            //handle attacks from knights
            let knight_attack_pattern = piece_attack_map.knight_attack_maps[king_position as usize];
            attacker_map = knight_attack_pattern & (game.piece_bitboards[2]  & game.occupancy_bitboards[1]);
            if attacker_map != 0 {
                check_type = check_type + 1;
                if check_type >= 2{return (2,0)}               
                defense_pattern = attacker_map;
            }

            //handle attacks from pawns
            let pawn_attack_pattern = piece_attack_map.pawn_attack_maps[0][king_position as usize];
            attacker_map = pawn_attack_pattern & (game.piece_bitboards[0] & game.occupancy_bitboards[1]);
            if attacker_map != 0 {
                check_type = check_type + 1;
                if check_type >= 2{return (2,0)}               
                defense_pattern = attacker_map ;
            }
        }
        PlayerColor::Black => {
            let bishop_attack_pattern = get_attacks_bishop(king_position, all_occupancies, 
                                                &piece_attack_map.bishop_relevant_occupancy, 
                                                &piece_attack_map.bishop_attack_maps);
            
            //handle diagonal attacks from bishops and queens
            attacker_map = bishop_attack_pattern & ((game.piece_bitboards[3] | game.piece_bitboards[4]) & game.occupancy_bitboards[0]);
            if attacker_map != 0 {
                unsafe {
                    attacker_position = get_lsb_index(attacker_map);
                }
                check_type = check_type + 1;
                opponent_attacker_pattern = get_attacks_bishop(attacker_position, all_occupancies, 
                                                        &piece_attack_map.bishop_relevant_occupancy, 
                                                        &piece_attack_map.bishop_attack_maps);
                defense_pattern = (bishop_attack_pattern & opponent_attacker_pattern) | attacker_map;
            }
            let rook_attack_pattern = get_attacks_rook(king_position, all_occupancies, 
                                                &piece_attack_map.rook_relevant_occupancy, 
                                                &piece_attack_map.rook_attack_maps);
            


            //handle straight attacks from rooks and queens
            attacker_map = rook_attack_pattern & ((game.piece_bitboards[1] | game.piece_bitboards[4]) & game.occupancy_bitboards[0]);
            if attacker_map != 0 {
                unsafe {
                    attacker_position = get_lsb_index(attacker_map);
                }
                check_type = check_type + 1;
                if check_type >= 2{return (2,0)}
                opponent_attacker_pattern = get_attacks_rook(attacker_position, all_occupancies, 
                                                        &piece_attack_map.rook_relevant_occupancy, 
                                                        &piece_attack_map.rook_attack_maps);
                defense_pattern = (rook_attack_pattern & opponent_attacker_pattern) | attacker_map;
            }

            //handle attacks from knights
            let knight_attack_pattern = piece_attack_map.knight_attack_maps[king_position as usize];
            attacker_map = knight_attack_pattern & (game.piece_bitboards[2]  & game.occupancy_bitboards[0]);
            if attacker_map != 0 {
                check_type = check_type + 1;
                if check_type >= 2{return (2,0)}               
                defense_pattern = attacker_map;
            }

            //handle attacks from pawns
            let pawn_attack_pattern = piece_attack_map.pawn_attack_maps[1][king_position as usize];
            attacker_map = pawn_attack_pattern & (game.piece_bitboards[0] & game.occupancy_bitboards[0]);
            if attacker_map != 0 {
                check_type = check_type + 1;
                if check_type >= 2{return (2,0)}               
                defense_pattern = attacker_map;
            }
        }
    }
    return (check_type,defense_pattern);
}   

//a function to find out the piece present on a particular square
#[inline(always)]
pub fn get_piece_at_sqaure(game:&game_state,square:u8) -> u32 {
    if get_bit!(game.piece_bitboards[0],square) == 1 {return PIECE_PAWN}
    if get_bit!(game.piece_bitboards[1],square) == 1 {return PIECE_ROOK}
    if get_bit!(game.piece_bitboards[2],square) == 1 {return PIECE_KNIGHT}
    if get_bit!(game.piece_bitboards[3],square) == 1 {return PIECE_BISHOP}
    if get_bit!(game.piece_bitboards[4],square) == 1 {return PIECE_QUEEN}
    if get_bit!(game.piece_bitboards[5],square) == 1 {return PIECE_KING}
    return PIECE_NONE
}


pub fn generate_moves(side:PlayerColor,game:&game_state,piece_attack_map:&attack_map) -> MoveList{
    let mut move_list = MoveList::new(side);
    let mut starting_square:u8;
    let mut ending_square:u8;
    let mut single_pawn_moves:u64;
    

    let all_occupancies = game.occupancy_bitboards[0] | game.occupancy_bitboards[1];
    let player_occupancies = match side {
        PlayerColor::White => {
            game.occupancy_bitboards[0]
        }
        PlayerColor::Black => {
            game.occupancy_bitboards[1]
        }
    };
    let opponent_occupancies = match side {
        PlayerColor::White => {
            game.occupancy_bitboards[1]
        }
        PlayerColor::Black => {
            game.occupancy_bitboards[0]
        }
    };
    
    let defense_map;
    let check_type;
    let king_position;

    //getting check status for king
    unsafe {
        king_position = get_lsb_index(game.piece_bitboards[5] & player_occupancies);
    }
    (check_type,defense_map) = get_check_type(side,&game,piece_attack_map,king_position,all_occupancies);


    // Capturing pawn moves
    let side_pawn_bitboard =game.piece_bitboards[0] & player_occupancies;
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
        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,PIECE_NONE,0,0,0,0,0));
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
        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,PIECE_NONE,0,0,1,0,0));

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
        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,PIECE_NONE,PROMOTION_QUEEN,1,0,0,0));
        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,PIECE_NONE,PROMOTION_ROOK,1,0,0,0));
        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,PIECE_NONE,PROMOTION_BISHOP,1,0,0,0));
        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,PIECE_NONE,PROMOTION_KNIGHT,1,0,0,0));
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
            let piece_captured = get_piece_at_sqaure(game, ending_square);
            //handling pawn capture promotions
            match side{
                PlayerColor::White => {
                    if ending_square >= 56 {
                        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,piece_captured,PROMOTION_QUEEN,1,0,0,0));
                        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,piece_captured,PROMOTION_KNIGHT,1,0,0,0));
                        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,piece_captured,PROMOTION_BISHOP,1,0,0,0));
                        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,piece_captured,PROMOTION_ROOK,1,0,0,0));
                        pawn_attack_map = pop_bit!(pawn_attack_map,ending_square);
                        continue;
                    }
                }
                PlayerColor::Black => {
                    if ending_square <= 7 {
                        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,piece_captured,PROMOTION_QUEEN,1,0,0,0));
                        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,piece_captured,PROMOTION_KNIGHT,1,0,0,0));
                        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,piece_captured,PROMOTION_BISHOP,1,0,0,0));
                        move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,piece_captured,PROMOTION_ROOK,1,0,0,0));
                        pawn_attack_map = pop_bit!(pawn_attack_map,ending_square);
                        continue;
                    }
                }
            }
            //handling normal pawn captures
            move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_PAWN,piece_captured,0,0,0,0,0));
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
                    move_list.add_move(encode_move!(starting_square as u32,en_pessant_square as u32,PIECE_PAWN,PIECE_PAWN,0,0,0,1,0));
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
    starting_square = king_position;
    if check_type == 0 {
        match side {
            PlayerColor::White => {
                //King Side Castling
                if (game.castle_flags & 0b0001) != 0 && (all_occupancies & WHITE_KING_SIDE_CASTLE) == 0
                                  && is_square_attacked(5, &PlayerColor::Black, 
                                                        &game, &piece_attack_map) == 0{
                    ending_square = 6;
                    move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_KING,PIECE_NONE,0,0,0,0,1));
                    }
                
                //Queen side Castling
                if (game.castle_flags & 0b0010) != 0 && (all_occupancies & WHITE_QUEEN_SIDE_CASTLE) == 0
                                  && is_square_attacked(3, &PlayerColor::Black, 
                                                        &game, &piece_attack_map) == 0{
                    ending_square = 2;
                    move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_KING,PIECE_NONE,0,0,0,0,1));


                }
            }
            PlayerColor::Black => {
                //King side Castling
                if (game.castle_flags & 0b0100) != 0 && (all_occupancies & BLACK_KING_SIDE_CASTLE) == 0
                                  && is_square_attacked(61, &PlayerColor::White, 
                                                        &game, &piece_attack_map) == 0{
                    ending_square = 62;
                    move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_KING,PIECE_NONE,0,0,0,0,1));
                }

                //Queen Side Castling
                if (game.castle_flags & 0b1000) != 0 && (all_occupancies & BLACK_QUEEN_SIDE_CASTLE) == 0
                                  && is_square_attacked(59, &PlayerColor::White, 
                                                        &game, &piece_attack_map) == 0{
                    ending_square = 58;
                    move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_KING,PIECE_NONE,0,0,0,0,1));

                }
            }
        }
    }

    //Handling Knight Moves
    let mut knight_bitboard = game.piece_bitboards[2] & player_occupancies;
    let mut knight_attack_pattern:u64;
    while knight_bitboard != 0{
        unsafe{
            starting_square = get_lsb_index(knight_bitboard);
        }
        
        knight_attack_pattern = piece_attack_map.knight_attack_maps[starting_square as usize] & (!player_occupancies);
        while knight_attack_pattern != 0 {
            unsafe{
                ending_square = get_lsb_index(knight_attack_pattern);
            }

            if(get_bit!(opponent_occupancies,ending_square)!=0){
                let piece_captured = get_piece_at_sqaure(game, ending_square);
                move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_KNIGHT,piece_captured,0,0,0,0,0));
            }
            else {
                move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_KNIGHT,PIECE_NONE,0,0,0,0,0));
            }
            knight_attack_pattern = pop_bit!(knight_attack_pattern,ending_square);
        }
        knight_bitboard = pop_bit!(knight_bitboard,starting_square);
    }

    //Handling Bishop Moves
    let mut bishop_bitboard = game.piece_bitboards[3] & player_occupancies;
    let mut bishop_attack_pattern:u64;
    while bishop_bitboard != 0{
        unsafe{
            starting_square = get_lsb_index(bishop_bitboard);
        }
        
        bishop_attack_pattern = get_attacks_bishop(starting_square, all_occupancies, 
                                                    &piece_attack_map.bishop_relevant_occupancy, 
                                                    &piece_attack_map.bishop_attack_maps) 
                                                    & (!player_occupancies);
        while bishop_attack_pattern != 0 {
            unsafe{
                ending_square = get_lsb_index(bishop_attack_pattern);
            }

            if(get_bit!(opponent_occupancies,ending_square)!=0){
                let piece_captured = get_piece_at_sqaure(game, ending_square);
                move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_BISHOP,piece_captured,0,0,0,0,0));
            }
            else {
                move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_BISHOP,PIECE_NONE,0,0,0,0,0));
            }
            bishop_attack_pattern = pop_bit!(bishop_attack_pattern,ending_square);
        }
        bishop_bitboard = pop_bit!(bishop_bitboard,starting_square);
    }

    //Handling Rook Moves
    let mut rook_bitboard = game.piece_bitboards[1] & player_occupancies;
    let mut rook_attack_pattern:u64;
    while rook_bitboard != 0{
        unsafe{
            starting_square = get_lsb_index(rook_bitboard);
        }
        
        rook_attack_pattern = get_attacks_rook(starting_square, all_occupancies, 
                                                    &piece_attack_map.rook_relevant_occupancy, 
                                                    &piece_attack_map.rook_attack_maps) 
                                                    & (!player_occupancies);
        while rook_attack_pattern != 0 {
            unsafe{
                ending_square = get_lsb_index(rook_attack_pattern);
            }

            if(get_bit!(opponent_occupancies,ending_square)!=0){
                let piece_captured = get_piece_at_sqaure(game, ending_square);
                move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_ROOK,piece_captured,0,0,0,0,0));
            }
            else {
                move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_ROOK,PIECE_NONE,0,0,0,0,0));
            }
            rook_attack_pattern = pop_bit!(rook_attack_pattern,ending_square);
        }
        rook_bitboard = pop_bit!(rook_bitboard,starting_square);
    }

    //Handling Queen Moves
    let mut queen_bitboard = game.piece_bitboards[4] & player_occupancies;
    let mut queen_attack_pattern:u64;
    while queen_bitboard != 0{
        unsafe{
            starting_square = get_lsb_index(queen_bitboard);
        }
        
        queen_attack_pattern = get_attacks_queen(starting_square, all_occupancies, 
                                                    &piece_attack_map.rook_relevant_occupancy, 
                                                    &piece_attack_map.rook_attack_maps,
                                                    &piece_attack_map.bishop_relevant_occupancy,
                                                    &piece_attack_map.bishop_attack_maps) 
                                                    & (!player_occupancies);
        while queen_attack_pattern != 0 {
            unsafe{
                ending_square = get_lsb_index(queen_attack_pattern);
            }

            if(get_bit!(opponent_occupancies,ending_square)!=0){
                let piece_captured = get_piece_at_sqaure(game, ending_square);
                move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_QUEEN,piece_captured,0,0,0,0,0));
            }
            else {
                move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_QUEEN,PIECE_NONE,0,0,0,0,0));
            }
            queen_attack_pattern = pop_bit!(queen_attack_pattern,ending_square);
        }
        queen_bitboard = pop_bit!(queen_bitboard,starting_square);
    }

    //Handling King Moves
    let mut king_bitboard = game.piece_bitboards[5] & player_occupancies;
    let mut king_attack_pattern:u64;
    while king_bitboard != 0{
        unsafe{
            starting_square = get_lsb_index(king_bitboard);
        }
        
        king_attack_pattern = piece_attack_map.king_attack_maps[starting_square as usize] & (!player_occupancies);
        while king_attack_pattern != 0 {
            unsafe{
                ending_square = get_lsb_index(king_attack_pattern);
            }

            if(get_bit!(opponent_occupancies,ending_square)!=0){
                let piece_captured = get_piece_at_sqaure(game, ending_square);
                move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_KING,piece_captured,0,0,0,0,0));
            }
            else {
                move_list.add_move(encode_move!(starting_square as u32,ending_square as u32,PIECE_KING,PIECE_NONE,0,0,0,0,0));
            }
            king_attack_pattern = pop_bit!(king_attack_pattern,ending_square);
        }
        king_bitboard = pop_bit!(king_bitboard,starting_square);
    }
    move_list
}