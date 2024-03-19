use std::thread;
use test_scr::test;

pub mod game_ui;
pub mod test_scr;
pub mod utils {
    pub mod util_enums;
    pub mod util_functions;
    pub mod util_constants;
}
pub mod engine {
    pub mod game_state;
    pub mod init;
    pub mod move_utils;
    pub mod attack_maps;
    pub mod bitboard_utils{
        pub mod base_operations;
        pub mod bitboard_constants;
        pub mod rng_utils;
        pub mod magic_bitboard_utils;
    }
}


fn main() -> Result<(), String> {
    test();
    //call game_ui
    //game_ui::init()?;
    Ok(())
}
