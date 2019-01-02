// mods
#[allow(dead_code)]
mod entities;
mod gamestate;
// remember to remove this dipshit
#[allow(dead_code)]
mod terminal;

const SHEET_SIZE: (usize, usize) = (8, 8);
const TERM_SIZE: (usize, usize) = (81, 40);
const WINDOW_SIZE: (u32, u32) = (
    TERM_SIZE.0 as u32 * SHEET_SIZE.0 as u32,
    TERM_SIZE.1 as u32 * SHEET_SIZE.1 as u32,
);

// namespacing
use quicksilver::lifecycle::{run, Settings};

fn main() {
    // run the gamestate
    run::<gamestate::GameState>("yeet", WINDOW_SIZE.into(), Settings::default());
}
