

use crate::utils::util_enums::PlayerColor;
use crate::utils::util_constants::{BOARD_START_X,BOARD_START_Y,SQR_SIZE,BOARD_WIDTH,BOARD_HEIGHT};
//a function to load board state from FEN string
pub fn load_fen(fen_string:&String) -> ([i32;64],u32,PlayerColor,bool,bool,bool,bool,u32,u32) {
    let mut player_move :PlayerColor = PlayerColor::White; 
    let mut game_board: [i32;64] = [0;64];
    let mut k_castling_w = false;
    let mut q_castling_w = false;
    let mut k_castling_b = false;
    let mut q_castling_b = false;
    let mut halfmove = 0;
    let mut fullmove = 0;
    let mut en_pessant_sqr = 65;
    let mut row = 7;
    let mut column = 0;
    let mut curr_ptr;
    let mut state = 0;
    for letter in fen_string.chars() {
        curr_ptr = row * 8 + column;
        if letter == ' ' {
            state+=1;
            continue;
        }
        match state {
            0 => {
                match letter {
                    'K' => {
                        game_board[curr_ptr] = 1;
                        column+=1
                    },
                    'Q' => {
                        game_board[curr_ptr] = 2;
                        column+=1
                    },
                    'B' => {
                        game_board[curr_ptr] = 3;
                        column+=1
                    },
                    'N' => {
                        game_board[curr_ptr] = 4;
                        column+=1
                    },
                    'R' => {
                        game_board[curr_ptr] = 5;
                        column+=1
                    },
                    'P' => {
                        game_board[curr_ptr] = 6;
                        column+=1
                    },
                    'k' => {
                        game_board[curr_ptr] = -1;
                        column+=1
                    },
                    'q' => {
                        game_board[curr_ptr] = -2;
                        column+=1
                    },
                    'b' => {
                        game_board[curr_ptr] = -3;
                        column+=1
                    },
                    'n' => {
                        game_board[curr_ptr] = -4;
                        column+=1
                    },
                    'r' => {
                        game_board[curr_ptr] = -5;
                        column+=1
                    },
                    'p' => {
                        game_board[curr_ptr] = -6;
                        column+=1
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
            1 => {
                match letter {
                    'w' => player_move = PlayerColor::White,
                    'b' => player_move = PlayerColor::Black,
                    _ => panic!("FEN provided is invalid")
                }
            }
            2 => {
                match letter {
                    'K' => k_castling_w = true,
                    'Q' => q_castling_w = true,
                    'k' => k_castling_b = true,
                    'q' => q_castling_b = true,
                    '-' => {},
                    _ => panic!("FEN provided is invalid")
                }
            }
            3 => {
                match letter {
                    'a'..='h' => en_pessant_sqr = letter as u32 - 'a' as u32,
                    '1'..='8' => en_pessant_sqr += (letter.to_digit(10).unwrap()-1)*8,
                    '-' => {},
                    _ => panic!("FEN provided is invalid")
                }
            }
            4 => {
                match letter {
                    '0'..='9' => halfmove = halfmove*10 + letter.to_digit(10).unwrap(),
                    _ => panic!("FEN provided is invalid")
                }
            }
            5 => {
                match letter {
                    '0'..='9' => fullmove = fullmove*10 + letter.to_digit(10).unwrap(),
                    _ => panic!("FEN provided is invalid")
                }
            }
            _ => panic!("FEN provided is invalid")

        }
        
    }
    (game_board,en_pessant_sqr,player_move,k_castling_w,q_castling_w,k_castling_b,q_castling_b,halfmove,fullmove)
}

//a function to get the coord on game state based on the x and y coordinate relative to the start of the board
pub fn get_position_from_coords(x_coord:i32,y_coord:i32,player_side:&PlayerColor) -> i32 {
    let col = (x_coord -BOARD_START_X as i32)/(SQR_SIZE as i32);
    let row = 7 - (y_coord-BOARD_START_Y as i32)/(SQR_SIZE as i32);
    if col<0 || col>7 || row<0 || row>7 {
        return 65;
    }
    match player_side {
        PlayerColor::White => return row*8 + col,
        PlayerColor::Black => return  (7-row)*8 + (7-col)
    }
}

//get the top left coordinate of the square based on the index position
pub fn get_coords_from_position(coords:i32,player_side:&PlayerColor) -> (i32,i32) {
    let row = coords/8;
    let col = coords%8;
    let x_coord;
    let y_coord;
    match player_side {
        PlayerColor::Black => {
            y_coord = (row)*(SQR_SIZE as i32) + BOARD_START_Y as i32;
            x_coord = (7 - col)*(SQR_SIZE as i32) + BOARD_START_X as i32;
        }
        PlayerColor::White => {
            y_coord = (7 - row)*(SQR_SIZE as i32) + BOARD_START_Y as i32;
            x_coord = (col)*(SQR_SIZE as i32) + BOARD_START_X as i32;
        }
    }
    (x_coord,y_coord)
}

pub fn is_in_range(x_coord:i32,y_coord:i32) -> bool{
    x_coord as u32 >= BOARD_START_X && x_coord as u32 <= BOARD_START_X+BOARD_WIDTH
                    && y_coord as u32 >= BOARD_START_Y && y_coord as u32 <= BOARD_START_Y+BOARD_HEIGHT
}