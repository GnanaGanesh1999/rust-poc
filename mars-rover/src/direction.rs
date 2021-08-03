use crate::coordinate::Coordinate;

pub enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    pub fn forward(&mut self, coordinate: &mut Coordinate) {
        match self {
            Direction::North => { coordinate.up() }
            Direction::East => { coordinate.right() }
            Direction::West => { coordinate.left() }
            Direction::South => { coordinate.down() }
        }
    }

    pub fn rotate_left(&mut self) -> Direction {
        match self {
            Direction::North => { Direction::West }
            Direction::East => { Direction::North }
            Direction::West => { Direction::South }
            Direction::South => { Direction::East }
        }
    }

    pub fn rotate_right(&mut self) -> Direction {
        match self {
            Direction::North => { Direction::East }
            Direction::East => { Direction::South }
            Direction::West => { Direction::North }
            Direction::South => { Direction::West }
        }
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::North => String::from("North"),
            Direction::South => String::from("South"),
            Direction::East => String::from("East"),
            Direction::West => String::from("West"),
        }
    }
}