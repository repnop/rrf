// namespacing
use crate::terminal;
use quicksilver::graphics::Color;
use quicksilver::lifecycle::State;
use quicksilver::lifecycle::Window;
use quicksilver::Result;

// main game state
pub struct GameState {
    pub terminal: terminal::AsciiTerminal,
}

impl State for GameState {
    fn new() -> Result<Self> {
        let terminal = terminal::AsciiTerminal::new("unscii8.png", (8, 8), (80, 40));
        Ok(GameState { terminal })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        self.terminal.render(window);

        Ok(())
    }
}
