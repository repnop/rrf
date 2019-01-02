use crate::entities::Entities;

#[derive(Debug)]
pub enum Turn {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    NoAction,
}

impl Turn {
    pub fn handle(&mut self, entities: &mut Entities, term_size: (usize, usize)) -> Turn {
        match self {
            Turn::MoveUp => {
                if entities[0].y > 0 {
                    entities[0].y -= 1;
                }
                Turn::NoAction
            }
            Turn::MoveDown => {
                if entities[0].y < term_size.1 as u32 - 1 {
                    entities[0].y += 1;
                }
                Turn::NoAction
            }
            Turn::MoveLeft => {
                if entities[0].x > 0 {
                    entities[0].x -= 1;
                }
                Turn::NoAction
            }
            Turn::MoveRight => {
                if entities[0].x < term_size.0 as u32 - 1 {
                    entities[0].x += 1;
                }
                Turn::NoAction
            }

            Turn::NoAction => Turn::NoAction,
        }
    }
}
