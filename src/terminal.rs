// namespacing
use quicksilver::{
    geom::Rectangle,
    graphics::{Background::Img, Color, Image},
    lifecycle::{Asset, Window},
};

// const
const ASCII_SHEET_DIM: (usize, usize) = (32, 7);

// an ascii terminal to represent the game
pub struct AsciiTerminal {
    // font resource in px
    pub font_size: (usize, usize),
    // terminal size in char
    pub terminal_size: (usize, usize),

    // the buffer of characters and colors
    pub char_buffer: Vec<CharColor>,

    // holds the font image asset
    pub font_image: Asset<Image>,
}

// maps a character and a color together for use on the terminal
#[derive(Clone, Copy)]
pub struct CharColor {
    // ascii character
    pub ascii_character: char,
    // corresponding quicksilver color
    pub qs_color: Color,
}

impl AsciiTerminal {
    // create a new terminal. buffer will be filled with blank chars
    pub fn new(
        font_filename: &'static str,
        font_size: (usize, usize),
        terminal_size: (usize, usize),
    ) -> Self {
        // init the character buffer and fill with empty chars
        let mut char_buffer: Vec<CharColor> = Vec::new();
        for _ in 0..(terminal_size.0 * terminal_size.1) {
            char_buffer.push(CharColor {
                ascii_character: ' ',
                qs_color: Color::BLACK,
            });
        }

        // load the font image to the struct
        let font_image = Asset::new(Image::load(font_filename));

        AsciiTerminal {
            font_size,
            terminal_size,
            char_buffer,
            font_image,
        }
    }

    // places specified char at location on the screen
    pub fn put_char(&mut self, loc: (usize, usize), ascii_character: char, qs_color: Color) {
        self[loc] = CharColor {
            ascii_character,
            qs_color,
        };
    }

    // places specified string starting at location
    pub fn put_str(&mut self, loc: (usize, usize), ascii_str: &'static str, qs_color: Color) {
        let ascii_chars: Vec<char> = ascii_str.chars().collect();
        for (i, &ch) in ascii_chars.iter().enumerate() {
            self.put_char((loc.0 + i, loc.1), ch, qs_color);
        }
    }

    // clear the buffer to be ' ' and black
    pub fn clear(&mut self) {
        // iterate through and write blank chars to buffer
        self.char_buffer.iter_mut().for_each(|cc| {
            *cc = CharColor {
                ascii_character: ' ',
                qs_color: Color::BLACK,
            }
        });
    }

    // render the console to the screen
    pub fn render(&mut self, window: &mut Window) {
        // draw all tiles to the window
        for x in 0..self.terminal_size.0 {
            for y in 0..self.terminal_size.1 {
                let cc = self[(x, y)];
                let font_size = (self.font_size.0 as f32, self.font_size.1 as f32);

                let _ = self.font_image.execute(|img| {
                    window.draw(
                        &Rectangle::new(
                            (x as f32 * font_size.0 - 1 as f32, y as f32 * font_size.1),
                            font_size,
                        ),
                        Img(&img.subimage(Rectangle::new(
                            char_to_coord(cc.ascii_character, font_size),
                            font_size,
                        ))),
                    );
                    Ok(())
                });
            }
        }
    }
}

// figures a coord on an ascii sheet given the char
fn char_to_coord(ascii_character: char, size: (f32, f32)) -> (f32, f32) {
    let ascii_character = ascii_character as usize - 32;
    (
        (ascii_character % ASCII_SHEET_DIM.0) as f32 * size.0,
        (ascii_character / ASCII_SHEET_DIM.0) as f32 * size.1,
    )
}

// indexing shit
mod indexing {
    use super::{AsciiTerminal, CharColor};
    use std::ops::{Index, IndexMut};

    impl Index<(usize, usize)> for AsciiTerminal {
        type Output = CharColor;

        fn index(&self, idx: (usize, usize)) -> &CharColor {
            &self.char_buffer[(idx.1 * self.terminal_size.0 + idx.0)]
        }
    }

    impl IndexMut<(usize, usize)> for AsciiTerminal {
        fn index_mut(&mut self, idx: (usize, usize)) -> &mut CharColor {
            &mut self.char_buffer[(idx.1 * self.terminal_size.0 + idx.0)]
        }
    }
}
