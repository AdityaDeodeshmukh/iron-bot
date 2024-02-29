

pub mod game_ui;
pub mod utils {
    pub mod util_enums;
    pub mod util_functions;
    pub mod util_constants;
}
pub mod engine {
    pub mod move_utils;
}
fn main() -> Result<(), String> {
    game_ui::init()?;
    println!("Hello, world!");
    Ok(())
}
