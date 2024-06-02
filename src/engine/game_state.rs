use crate::engine::bitboard_utils::base_operations::get_bit;

use super::bitboard_utils::base_operations::set_bit;


/*
game_state struct ----->
piece_bitboards:   [Pawns, Rooks, Knights, Bishops, Queen, King]
Occupancy_bitboards: [White Occupancies, Black Occupancies]
en_pessant_sqaure: Square on which en pessant is possible (65 if not possible)
side_to_move: true if white's move, false if black's move
castle_flags : 
0001 -> White King Side Castle
0010 -> White Queen Side Castle
0100 -> Black King Side Castle
1000 -> Black Queen Side Castle
 */
pub struct game_state {
    pub piece_bitboards:[u64;6],
    pub occupancy_bitboards:[u64;2],
    pub en_pessant_square:u8,
    pub side_to_move:bool,
    pub castle_flags:u8,
   
}

impl game_state {
    //generate an empty game state
    pub fn new() -> game_state {
        game_state {
            piece_bitboards:[0;6],
            occupancy_bitboards:[0;2],
            en_pessant_square:65,
            side_to_move:true,
            castle_flags:0000
        }
    }
    
    //load a game state from a particular FEN
    pub fn new_from_fen(fen_string:&String) -> game_state {
        let mut piece_bitboards:[u64;6] = [0;6];
        let mut occupancy_bitboards:[u64;2] = [0;2];
        //let ascii_codes = ['P','R','N','B','Q','K','p','r','n','b','q','k'];
        let mut side_to_move = true; 
        let mut en_pessant_square:u8 = 65;
        let mut row = 7;
        let mut column = 0;
        let mut curr_ptr;
        let mut state = 0;
        let mut castle_flags = 0;
        //iterate over characters
        for letter in fen_string.chars() {
            curr_ptr = row * 8 + column;
            if letter == ' ' {
                state+=1;
                continue;
            }
            match state {
                //0 state is getting the board position
                0 => {
                    match letter {
                        //White King
                        'K' => {
                            piece_bitboards[5] = set_bit!(piece_bitboards[5],curr_ptr);
                            occupancy_bitboards[0] = set_bit!(occupancy_bitboards[0],curr_ptr);
                            column+=1;
                        },
                        //White Queen
                        'Q' => {
                            piece_bitboards[4] = set_bit!(piece_bitboards[4],curr_ptr);
                            occupancy_bitboards[0] = set_bit!(occupancy_bitboards[0],curr_ptr);
                            column+=1;
                        },
                        //White Bishop
                        'B' => {
                            piece_bitboards[3] = set_bit!(piece_bitboards[3],curr_ptr);
                            occupancy_bitboards[0] = set_bit!(occupancy_bitboards[0],curr_ptr);
                            column+=1;
                        },
                        //White Knight
                        'N' => {
                            piece_bitboards[2] = set_bit!(piece_bitboards[2],curr_ptr);
                            occupancy_bitboards[0] = set_bit!(occupancy_bitboards[0],curr_ptr);
                            column+=1;
                        },
                        //White Rook
                        'R' => {
                            piece_bitboards[1] = set_bit!(piece_bitboards[1],curr_ptr);
                            occupancy_bitboards[0] = set_bit!(occupancy_bitboards[0],curr_ptr);
                            column+=1;
                        },
                        //White Pawn
                        'P' => {
                            piece_bitboards[0] = set_bit!(piece_bitboards[0],curr_ptr);
                            occupancy_bitboards[0] = set_bit!(occupancy_bitboards[0],curr_ptr);
                            column+=1;
                        },
                        //Black King
                        'k' => {
                            piece_bitboards[5] = set_bit!(piece_bitboards[5],curr_ptr);
                            occupancy_bitboards[1] = set_bit!(occupancy_bitboards[1],curr_ptr);
                            column+=1;
                        },
                        //Black Queen
                        'q' => {
                            piece_bitboards[4] = set_bit!(piece_bitboards[4],curr_ptr);
                            occupancy_bitboards[1] = set_bit!(occupancy_bitboards[1],curr_ptr);
                            column+=1;
                        },
                        //Black Bishop
                        'b' => {
                            piece_bitboards[3] = set_bit!(piece_bitboards[3],curr_ptr);
                            occupancy_bitboards[1] = set_bit!(occupancy_bitboards[1],curr_ptr);
                            column+=1;
                        },
                        //Black Knight
                        'n' => {
                            piece_bitboards[2] = set_bit!(piece_bitboards[2],curr_ptr);
                            occupancy_bitboards[1] = set_bit!(occupancy_bitboards[1],curr_ptr);
                            column+=1;
                        },
                        //Black Rook
                        'r' => {
                            piece_bitboards[1] = set_bit!(piece_bitboards[1],curr_ptr);
                            occupancy_bitboards[1] = set_bit!(occupancy_bitboards[1],curr_ptr);
                            column+=1;
                        },
                        //Black Pawn
                        'p' => {
                            piece_bitboards[0] = set_bit!(piece_bitboards[0],curr_ptr);
                            occupancy_bitboards[1] = set_bit!(occupancy_bitboards[1],curr_ptr);
                            column+=1;
                        },
                        '0'..='9' => {
                            let offset = letter.to_digit(10).unwrap() as usize;
                            column += offset;
                        },
                        '/' => {
                            row -= 1;
                            column = 0;
                        },
                        _ => panic!("FEN provided is invalid")
                    }
                    
                }
                //1 state gets which player's move it is
                1 => {
                    match letter {
                        'w' => side_to_move = true,
                        'b' => side_to_move = false,
                        _ => panic!("FEN provided is invalid")
                    }
                }
                //2 state gets castling info
                2 => {
                    match letter {
                        'K' => castle_flags |= 0b0001,
                        'Q' => castle_flags |= 0b0010,
                        'k' => castle_flags |= 0b0100,
                        'q' => castle_flags |= 0b1000,
                        '-' => {},
                        _ => panic!("FEN provided is invalid")
                    }
                }
                //3 state gets the en pessant square
                3 => {
                    match letter {
                        'a'..='h' => en_pessant_square = (letter as u32 - 'a' as u32) as u8,
                        '1'..='8' => en_pessant_square += ((letter.to_digit(10).unwrap()-1)*8) as u8,
                        '-' => {},
                        _ => panic!("FEN provided is invalid")
                    }
                }
                //4 state gets the halfmove (not implemented)
                4 => {
                    match letter {
                        '0'..='9' => {},
                        _ => panic!("FEN provided is invalid")
                    }
                }
                //5 state gets the fullmove (not implemented)
                5 => {
                    match letter {
                        '0'..='9' => {},
                        _ => panic!("FEN provided is invalid")
                    }
                }
                _ => panic!("FEN provided is invalid")

            }
            
        }
        game_state {
            piece_bitboards,
            occupancy_bitboards,
            en_pessant_square,
            side_to_move,
            castle_flags
        }
    }

    
    pub fn print_game_state(&self,unicode:bool) {
        let unicode_chars = ['♟','♜','♞','♝','♛','♚','♙','♖','♘','♗','♕','♔'];
        let ascii_chars =  ['P','R','N','B','Q','K','p','r','n','b','q','k'];
        let castle_wk = if (self.castle_flags & 0b0001) != 0 {true} else {false};
        let castle_wq = if (self.castle_flags & 0b0010) != 0 {true} else {false};
        let castle_bk = if (self.castle_flags & 0b0100) != 0 {true} else {false};
        let castle_bq = if (self.castle_flags & 0b1000) != 0 {true} else {false};
        for row in (0..=7).rev() {
            print!("{}   ",row+1);
            for file in 0..=7 {
                let square = row*8 + file;
                let mut piece = -1;
                for (i,bitboard) in self.piece_bitboards.iter().enumerate() {
                    if get_bit!(bitboard,square) != 0 {
                        if get_bit!(self.occupancy_bitboards[1],square) == 0 {
                            piece = i as i32;
                        }
                        else {
                            piece = (i as i32) + 6;
                        }
                        
                    }
                }
                let ch;
                if unicode {
                    ch = if piece==-1 {'\u{00B7}'} else {unicode_chars[piece as usize]};
                }
                else {
                    ch = if piece==-1 {'.'} else {ascii_chars[piece as usize]};
                }
                
                print!(" {} ",ch);
                
            }
            println!("");
        }
        println!("");
        println!("     a  b  c  d  e  f  g  h");
        println!("       Side to move: {}",if self.side_to_move{"White"} else {"Black"});
        println!("         En Pessant: {}",if self.en_pessant_square!=65{self.en_pessant_square.to_string()}else{"No".to_string()});
        print!("         Castling: {}", if castle_wk{'K'} else {'-'});
        print!("{}{}{}", if castle_wq{'Q'} else {'-'},if castle_bk{'k'} else {'-'},if castle_bq{'q'} else {'-'});
        println!("\n")
    }
}