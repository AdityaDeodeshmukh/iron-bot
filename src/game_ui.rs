extern crate sdl2;
use sdl2::event::Event;
use sdl2::render::{Canvas, Texture};
use sdl2::keyboard::Keycode;
use sdl2::render::BlendMode;
use sdl2::rect::Rect;
use sdl2::mouse::MouseButton;
use sdl2::video::Window;
use core::panic;
use std::path::Path;
use sdl2::image::LoadTexture;

use crate::utils::util_enums::PlayerColor;
use crate::utils::util_functions::{get_coords_from_position, get_position_from_coords, is_in_range, load_fen};
use crate::utils::util_constants::{BOARD_HEIGHT,BOARD_WIDTH,BOARD_START_X,BOARD_START_Y,SQR_SIZE,SCR_WD,SCR_HT};
use crate::engine::move_utils::{is_move_legal,change_board};

//defining main game loop
pub fn init() -> Result<(),String>{
    let player_color = PlayerColor::Black;
    
    println!("Hello");
    let theme = "./themes/default/";
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = match video
                        .window("Chess", SCR_WD, SCR_HT)
                        .position_centered()
                        .opengl()
                        .build() {
                        Ok(window) => window,
                        Err(err) => panic!("Failed to create window: {}",err)
                    };
    
    
    let mut canvas = window.into_canvas()
                                        .software()
                                        .build()
                                        .map_err(|e| e.to_string())?;
    canvas.set_blend_mode(BlendMode::Blend);
    
    let texture_creator = canvas.texture_creator();
    
    //loading textures for pieces
    let texture_w_k = texture_creator.load_texture(format!("{}W_King.png",theme))?;
    let texture_w_q = texture_creator.load_texture(format!("{}W_Queen.png",theme))?;
    let texture_w_kn = texture_creator.load_texture(format!("{}W_Knight.png",theme))?;
    let texture_w_b = texture_creator.load_texture(format!("{}W_Bishop.png",theme))?;
    let texture_w_r = texture_creator.load_texture(format!("{}W_Rook.png",theme))?;
    let texture_w_p = texture_creator.load_texture(format!("{}W_Pawn.png",theme))?;


    let texture_b_k = texture_creator.load_texture(format!("{}B_King.png",theme))?;
    let texture_b_q = texture_creator.load_texture(format!("{}B_Queen.png",theme))?;
    let texture_b_kn = texture_creator.load_texture(format!("{}B_Knight.png",theme))?;
    let texture_b_b = texture_creator.load_texture(format!("{}B_Bishop.png",theme))?;
    let texture_b_r = texture_creator.load_texture(format!("{}B_Rook.png",theme))?;
    let texture_b_p = texture_creator.load_texture(format!("{}B_Pawn.png",theme))?;
    let fen_str = String::from("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/KNBQKBNK b KQkq e3 0 1");
    let mut game_state: ([i32; 64], u32, PlayerColor, bool, bool, bool, bool, u32, u32) = load_fen(&fen_str);
    
    canvas.clear();
    let back_path = format!("{}Back.png",theme);
    let board_path = format!("{}ChessBoard.jpg",theme);
    let background_texture = texture_creator.load_texture(Path::new(&back_path))?;
    canvas.copy(&background_texture, None, None)?;
    let board_texture = texture_creator.load_texture(Path::new(&board_path))?;
    let board_rect = Rect::new(BOARD_START_X as i32,BOARD_START_Y as i32,BOARD_WIDTH,BOARD_HEIGHT);
    let mut is_dragging = false;
    let mut piece_index:u32 = 65;
    let mut x_coord_dragged_piece = 0;
    let mut y_coord_dragged_piece = 0;
    let mut has_state_changed = true;
    match draw_board(&mut canvas, &board_texture, board_rect, &player_color) {
        Ok(_) => {}
        Err(_) => panic!("Could not load board")
     }
    for (index,element) in game_state.0.iter().enumerate() {
        if index%8 == 0{
            println!(" ")
        }
        print!("{}   ",element);
        
    }
    

    //Quit the game using escape key or X button
    let mut events = context.event_pump()?;
    let mut main_loop = ||{
        for event in events.poll_iter(){
            match event{
                Event::Quit { .. }
                | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
                } => return true,
                _ => {}
            }
        }
        let mouse_state = events.mouse_state();
        let mouse_state_left = mouse_state.is_mouse_button_pressed(MouseButton::Left);

        //Code when mouse is pressed
        if mouse_state_left && !is_dragging {
            is_dragging = true;
            x_coord_dragged_piece = mouse_state.x();   
            y_coord_dragged_piece = mouse_state.y();
            if is_in_range(x_coord_dragged_piece, y_coord_dragged_piece) {
                    piece_index = get_position_from_coords(x_coord_dragged_piece, 
                                    y_coord_dragged_piece, &player_color) as u32;
                    has_state_changed = true;
                    if game_state.0[piece_index as usize] == 0 {
                        piece_index = 65;
                    }
            }
        }

        //When mouse is being dragged
        if mouse_state_left && is_dragging {
            has_state_changed = true;
        }

        //When mouse press is left
        if is_dragging && !mouse_state_left {
            has_state_changed = true;
            is_dragging = false;
            if piece_index < 65 {
                x_coord_dragged_piece = mouse_state.x();   
                y_coord_dragged_piece = mouse_state.y();
                if is_in_range(x_coord_dragged_piece, y_coord_dragged_piece) {
                        let final_idx = get_position_from_coords(x_coord_dragged_piece, 
                                            y_coord_dragged_piece, &player_color) as u32;
                        if is_move_legal(piece_index, final_idx) {
                            change_board(&mut game_state, piece_index, final_idx);
                            
                        }
                }
            }
            piece_index = 65;
        }
        //only update the canvas if a change has been detected (To save computation)
        if has_state_changed{
            has_state_changed = false;
            canvas.clear();
            match canvas.copy(&background_texture, None, None) {
                Ok(_) => {}
                Err(_) => panic!("Could not load background")
            };
            match draw_board(&mut canvas, &board_texture, board_rect, &player_color) {
                Ok(_) => {}
                Err(_) => panic!("Could not load board")
            }
            
            for (index,element) in game_state.0.iter().enumerate() {
                let mut x_coord;
                let mut y_coord;
                (x_coord,y_coord) = get_coords_from_position(index as i32, &player_color);
                if index == piece_index as usize{
                    x_coord = mouse_state.x() - (SQR_SIZE as i32)/2;
                    y_coord = mouse_state.y() - (SQR_SIZE as i32)/2;
                }
                match element {
                    0 => {},
                    1 => draw_piece(x_coord, y_coord, &texture_w_k, &mut canvas),
                    2 => draw_piece(x_coord, y_coord, &texture_w_q, &mut canvas),
                    3 => draw_piece(x_coord, y_coord, &texture_w_b, &mut canvas),
                    4 => draw_piece(x_coord, y_coord, &texture_w_kn, &mut canvas),
                    5 => draw_piece(x_coord, y_coord, &texture_w_r, &mut canvas),
                    6 => draw_piece(x_coord, y_coord, &texture_w_p, &mut canvas),
                    -1 => draw_piece(x_coord, y_coord, &texture_b_k, &mut canvas),
                    -2 => draw_piece(x_coord, y_coord, &texture_b_q, &mut canvas),
                    -3 => draw_piece(x_coord, y_coord, &texture_b_b, &mut canvas),
                    -4 => draw_piece(x_coord, y_coord, &texture_b_kn, &mut canvas),
                    -5 => draw_piece(x_coord, y_coord, &texture_b_r, &mut canvas),
                    -6 => draw_piece(x_coord, y_coord, &texture_b_p, &mut canvas),
                    _ => panic!("Element does not exist")
                }
            }

            canvas.present();
        }
        events.wait_event_timeout(10);
        false
    };
    loop{if main_loop(){
        break;
    }}
    Ok(())
}

fn draw_board(canvas:&mut Canvas<Window>,board_texture:&Texture,board_rect:Rect,player_color:&PlayerColor) -> Result<(), String> {
    match player_color{
        PlayerColor::White => canvas.copy(&board_texture,None,board_rect),
        PlayerColor::Black => {
            let texture_center_x = BOARD_WIDTH/ 2;
            let texture_center_y = BOARD_HEIGHT/ 2;
            canvas.copy_ex(&board_texture, None, board_rect, 180.0, 
                Some((texture_center_x as i32, texture_center_y as i32).into()), false, false)
        }
    }
}
fn draw_piece(x_coord:i32,y_coord:i32, texture:&Texture,canvas:&mut Canvas<Window>) {
    let piece_rect = Rect::new(x_coord, y_coord, SQR_SIZE, SQR_SIZE);
    canvas.copy(texture, None, piece_rect).unwrap();
}

