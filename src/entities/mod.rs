use crate::terminal::AsciiTerminal;
use quicksilver::graphics::Color;

// represents a set of entities
#[derive(Debug)]
pub struct Entities {
    pub entities: Vec<Entity>,
}

impl Entities {
    // initializs the entities vec
    pub fn init() -> Self {
        let entities: Vec<Entity> = Vec::new();
        Entities { entities }
    }

    // creates the player ent
    pub fn create_player(&mut self, loc: (u32, u32)) {
        let player = Entity::new("Player".to_string(), '@', Color::WHITE, loc);
        self.entities.push(player);
    }

    // adds the entities to a terminal
    pub fn update_term(&mut self, terminal: &mut AsciiTerminal) {
        for entity in &self.entities {
            terminal.put_char(
                (entity.x as usize, entity.y as usize),
                entity.ascii_char,
                entity.qs_color,
            );
        }
    }
}

// represents an entity in the game world
#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub ascii_char: char,
    pub qs_color: Color,
    pub x: u32,
    pub y: u32,
}

impl Entity {
    // create a new entity
    pub fn new(name: String, ascii_char: char, qs_color: Color, (x, y): (u32, u32)) -> Self {
        Entity {
            name,
            ascii_char,
            qs_color,
            x,
            y,
        }
    }
}

// indexing shit
mod indexing {
    use super::{Entities, Entity};
    use std::ops::{Index, IndexMut};

    impl Index<usize> for Entities {
        type Output = Entity;

        fn index(&self, idx: usize) -> &Entity {
            &self.entities[idx]
        }
    }

    impl IndexMut<usize> for Entities {
        fn index_mut(&mut self, idx: usize) -> &mut Entity {
            &mut self.entities[idx]
        }
    }
}
