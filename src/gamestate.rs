// namespacing
use crate::entities;
use crate::terminal;
use quicksilver::{
    graphics::Color,
    lifecycle::{State, Window},
    Result,
};

// const
const SCREEN_CENTER: (usize, usize) = (crate::TERM_SIZE.0 / 2, crate::TERM_SIZE.1 / 2);

// main game state
pub struct GameState {
    pub terminal: terminal::AsciiTerminal,
    pub entities: entities::Entities,
}

impl State for GameState {
    fn new() -> Result<Self> {
        // init terminal
        let terminal =
            terminal::AsciiTerminal::new("unscii8.png", crate::SHEET_SIZE, crate::TERM_SIZE);

        // init entities
        let mut entities = entities::Entities::init();
        entities.create_player((SCREEN_CENTER.0 as u32, SCREEN_CENTER.1 as u32));

        Ok(GameState { terminal, entities })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        // clear terminal
        self.terminal.clear();
        // write entities to terminal
        self.entities.update_term(&mut self.terminal);

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // clear the window
        window.clear(Color::BLACK)?;
        // draw the terminal to the screen
        self.terminal.render(window);

        Ok(())
    }
}
