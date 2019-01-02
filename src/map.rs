use crate::terminal::AsciiTerminal;
use quicksilver::graphics::Color;

// this is for later
/*#[derive(Debug)]
pub struct World {
    pub floors: Vec<Floor>,
}*/

// whole floor of tiles
#[derive(Debug)]
pub struct Floor {
    pub size: (usize, usize),
    pub tiles: Vec<Tile>,
}

impl Floor {
    // creates a new floor
    pub fn new(size: (usize, usize)) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();
        for _ in 0..(size.0 * size.1) {
            tiles.push(Tile::new(TileType::Nothing));
        }

        Floor { size, tiles }
    }

    // generates a test floor
    pub fn init_test_floor(&mut self) {
        let offset = crate::SCREEN_CENTER;
        for x in 0..8 {
            for y in 0..8 {
                self[(x + offset.0 - 4, y + offset.1 - 4)] = Tile::new(TileType::Ground);
                if x == 0 || x == 7 {
                    self[(x + offset.0 - 4, y + offset.1 - 4)] = Tile::new(TileType::Wall);
                } else if y == 0 || y == 7 {
                    self[(x + offset.0 - 4, y + offset.1 - 4)] = Tile::new(TileType::Wall);
                }
            }
        }
    }

    // renders the map to the terminal
    pub fn update_term(&mut self, terminal: &mut AsciiTerminal) {
        for x in 0..(self.size.0 - 1) {
            for y in 0..(self.size.1 - 1) {
                let ascii_char = match self[(x, y)].tile_type {
                    TileType::Ground => '.',
                    TileType::Wall => '#',
                    TileType::Nothing => ' ',
                };

                terminal.put_char((x, y), ascii_char, Color::WHITE);
            }
        }
    }
}

// single tile on the floor
#[derive(Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub blocking: bool,
}

impl Tile {
    // new tile from tile type
    pub fn new(tile_type: TileType) -> Self {
        let blocking: bool = match tile_type {
            TileType::Wall => true,
            TileType::Ground => false,
            TileType::Nothing => false,
        };

        Tile {
            blocking,
            tile_type,
        }
    }
}

// represents all types of tiles
#[derive(Debug)]
pub enum TileType {
    Wall,
    Ground,
    Nothing,
}

// indexing shit
mod indexing {
    use super::{Floor, Tile};
    use std::ops::{Index, IndexMut};

    impl Index<(usize, usize)> for Floor {
        type Output = Tile;

        fn index(&self, idx: (usize, usize)) -> &Tile {
            &self.tiles[(idx.1 * self.size.0 + idx.0)]
        }
    }

    impl IndexMut<(usize, usize)> for Floor {
        fn index_mut(&mut self, idx: (usize, usize)) -> &mut Tile {
            &mut self.tiles[(idx.1 * self.size.0 + idx.0)]
        }
    }
}
