// namespacing
use crate::{entities::Entities, map::Floor, terminal::AsciiTerminal, turn::Turn};
use quicksilver::{
    graphics::Color,
    input::{ButtonState, Key},
    lifecycle::{Event, State, Window},
    Result,
};

// main game state
pub struct GameState {
    pub terminal: AsciiTerminal,
    pub entities: Entities,
    pub turn: Turn,
    pub map: Floor,
}

impl State for GameState {
    fn new() -> Result<Self> {
        // init terminal
        let terminal = AsciiTerminal::new("unscii8.png", crate::SHEET_SIZE, crate::TERM_SIZE);

        // init entities
        let mut entities = Entities::init();
        entities.create_player((crate::SCREEN_CENTER.0 as u32, crate::SCREEN_CENTER.1 as u32));

        // init the turn enum
        let turn: Turn = Turn::NoAction;

        // init the floor
        let mut map: Floor = Floor::new(crate::TERM_SIZE);
        map.init_test_floor();

        Ok(GameState {
            terminal,
            entities,
            turn,
            map,
        })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        // handle the turn options
        self.turn = self
            .turn
            .handle(&mut self.entities, self.terminal.terminal_size);

        // clear terminal
        self.terminal.clear();
        // write map to the terminal
        self.map.update_term(&mut self.terminal);
        // write the entities to the terminal
        self.entities.update_term(&mut self.terminal);

        Ok(())
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        self.turn = match event {
            Event::Key(Key::Up, ButtonState::Pressed) => Turn::MoveUp,
            Event::Key(Key::Down, ButtonState::Pressed) => Turn::MoveDown,
            Event::Key(Key::Left, ButtonState::Pressed) => Turn::MoveLeft,
            Event::Key(Key::Right, ButtonState::Pressed) => Turn::MoveRight,

            _ => Turn::NoAction,
        };

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
