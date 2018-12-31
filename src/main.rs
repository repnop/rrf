// mods
mod gamestate;
// remember to remove this dipshit
#[allow(dead_code)]
mod terminal;

const WINDOW_SIZE: (u32, u32) = (80 * 8, 40 * 8);

// namespacing
use quicksilver::lifecycle::{run, Settings};

fn main() {
    // run the gamestate
    run::<gamestate::GameState>("yeet", WINDOW_SIZE.into(), Settings::default());
}
