/*
Move Definition:
Move:
Initial Square:         0000 0000 0000 0000 0011 1111   0x3F
Final Square:           0000 0000 0000 1111 1100 0000   0xFC0
Piece Moved:            0000 0000 0111 0000 0000 0000   0x7000
Piece Captured Flag:    0000 0011 1000 0000 0000 0000   0x38000
Promoted Piece Flag:    0000 1100 0000 0000 0000 0000   0xC0000
Promotion Flag:         0001 0000 0000 0000 0000 0000   0x100000
Double Push Flag:       0010 0000 0000 0000 0000 0000   0x200000
En Pessant Flag:        0100 0000 0000 0000 0000 0000   0x400000
Castling Flag:          1000 0000 0000 0000 0000 0000   0x800000
*/
pub struct MoveList {
    moves:[u32;256],
    count:u8,
    player: PlayerColor
}
impl MoveList {
    pub fn new(player_color: PlayerColor) -> MoveList {
        MoveList {
            moves: [0;256],
            count: 0,
            player: player_color
        }
    }

    pub fn add_move(&mut self,chess_move:u32){
        self.moves[self.count as usize] = chess_move;
        self.count += 1;
    }

    pub fn print_move_list(&self,is_unicode: bool) {
        println!("Move        Piece_Moved   Piece_Captured    Is Promotion    Double    Enpass    castling");
        if self.count == 0 {
            println!("            No moves in the movelist");
            return;
        }
        for i in 0..(self.count) {
            print_move(self.moves[i as usize], is_unicode,&self.player)
        }
        println!("Total Moves:{}",self.count);

    }
}



pub fn print_move(chess_move: u32, is_unicode: bool, player: &PlayerColor) {
    let starting_square = decode_initial_square!(chess_move) as u8;
    let ending_square = decode_final_square!(chess_move) as u8;
    let unicode_chars = ['♟','♜','♞','♝','♛','♚','♙','♖','♘','♗','♕','♔'];
    let promoted_chars = ['Q','R','N','B','q','r','n','b'];
    let ascii_chars =  ['P','R','N','B','Q','K','p','r','n','b','q','k'];
    let double_push = decode_double_push!(chess_move);
    let enpass = decode_en_pessant!(chess_move);
    let castle = decode_castling!(chess_move);
    let is_promotion = decode_promotion!(chess_move) as u8;
    let promoted_piece = decode_piece_promoted!(chess_move);
    let piece_moved = match player {
        PlayerColor::White => {
            if is_unicode {unicode_chars[decode_piece_moved!(chess_move) as usize]} 
                            else {ascii_chars[decode_piece_moved!(chess_move) as usize]}
        }
        PlayerColor::Black => {
            if is_unicode {unicode_chars[(decode_piece_moved!(chess_move)+6) as usize]} 
                            else {ascii_chars[(decode_piece_moved!(chess_move)+6) as usize]}
        }
        };
    let promotion_char = match player {
        PlayerColor::White => {
            if is_promotion != 0 {promoted_chars[promoted_piece as usize]} else {' '}
            
        }
        PlayerColor::Black => {
            if is_promotion != 0 {promoted_chars[(promoted_piece+4) as usize]} else {' '}
        }
    };
    let capture_code = decode_piece_captured!(chess_move);
    let piece_captured = if capture_code == 7 {'-'} else {match player {
        PlayerColor::White => {
            if is_unicode {unicode_chars[(capture_code+6) as usize]}
                             else {ascii_chars[(capture_code+6) as usize]}
        }
        PlayerColor::Black => {
            if is_unicode {unicode_chars[capture_code as usize]} 
                            else {ascii_chars[capture_code as usize]}
        }
        }};
    let is_capture = if capture_code == 7 {0} else {1};
    let is_promotion = decode_promotion!(chess_move);
    
    println!("{}{}{}{}{}       {}             {}                 {}               {}         {}         {}",
                            ((starting_square%8+b'a') as char),((starting_square/8+b'1') as char),
                            ((ending_square%8+b'a') as char),((ending_square/8+b'1') as char), promotion_char,
                                piece_moved,if is_capture==1 {piece_captured} else {'-'},is_promotion,
                            double_push,enpass,castle);
    
}

// Macro to encode the move
macro_rules! encode_move {
    ($initial_square:expr,$final_square:expr,$piece_moved:expr,
        $piece_captured:expr,$piece_promoted:expr,$is_promotion:expr,$double_push:expr,$en_pessant:expr,$castling:expr) => {
        $initial_square |
        $final_square << 6 |
        $piece_moved << 12 |
        $piece_captured << 15 |
        $piece_promoted << 18 |
        $is_promotion << 20 |
        $double_push << 21 |
        $en_pessant << 22 |
        $castling << 23
    };
}
pub(crate) use encode_move;


//Macros to decode information from moves
macro_rules! decode_initial_square {
    ($chess_move:expr) => {
        $chess_move & 0x3F        
    };
}
pub(crate) use decode_initial_square;


macro_rules! decode_final_square {
    ($chess_move:expr) => {
        ($chess_move & 0xFC0) >> 6        
    };
}
pub(crate) use decode_final_square;


macro_rules! decode_piece_moved {
    ($chess_move:expr) => {
        ($chess_move & 0x7000) >> 12
    };
}
pub(crate) use decode_piece_moved;


macro_rules! decode_piece_captured {
    ($chess_move:expr) => {
        ($chess_move & 0x38000) >> 15
    };
}
pub(crate) use decode_piece_captured;


macro_rules! decode_piece_promoted {
    ($chess_move:expr) => {
        ($chess_move & 0xC0000) >> 18
    };
}
pub(crate) use decode_piece_promoted;


macro_rules! decode_promotion {
    ($chess_move:expr) => {
        ($chess_move & 0x100000) >> 20
    };
}
pub(crate) use decode_promotion;


macro_rules! decode_double_push {
    ($chess_move:expr) => {
        ($chess_move & 0x200000) >> 21
    };
}
pub(crate) use decode_double_push;


macro_rules! decode_en_pessant {
    ($chess_move:expr) => {
        ($chess_move & 0x400000) >> 22
    };
}
pub(crate) use decode_en_pessant;


macro_rules! decode_castling {
    ($chess_move:expr) => {
        ($chess_move & 0x800000) >> 23
    };
}
pub(crate) use decode_castling;

use crate::utils::util_enums::PlayerColor;
