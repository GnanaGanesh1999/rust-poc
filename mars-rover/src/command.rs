use crate::position::Position;

pub enum Command {
    MOVE,
    LEFT,
    RIGHT,
}

impl Command {
    pub fn execute(&self, position: &mut Position) {
        match self {
            Command::MOVE => { position.forward() }
            Command::LEFT => { position.turn_left() }
            Command::RIGHT => { position.turn_right() }
        }
    }
}