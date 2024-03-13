use crate::utils::util_enums::PlayerColor;
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