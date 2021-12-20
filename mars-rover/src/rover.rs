use crate::command::Command;
use crate::direction::Direction;
use crate::position::Position;

pub struct Rover {
    position: Position,
}

impl Rover {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        let position = Position::new(x, y, direction);
        Self { position }
    }

    pub(crate) fn execute_commands(&mut self, commands: &Vec<Command>) {
        for command in commands {
            command.execute(&mut self.position);
        }
    }
}

impl ToString for Rover {
    fn to_string(&self) -> String {
        let coordinate = &self.position.coordinate;
        let mut out = String::new();
        out.push_str(&coordinate.x.to_string());
        out.push_str(", ".into());
        out.push_str(&coordinate.y.to_string());
        out.push_str(" : ".into());
        out.push_str(&self.position.direction.to_string());
        out.push_str("\n".into());
        out
    }
}

// public String get_position_info() {
// return position.toString();
// }
//
// public class Rover {
// private final Position position;
//
// public Rover(Integer x, Integer y, Direction direction) {
// this.position = new Position(x, y, direction);
// }
//
// public void execute_commands(List<Command> commands) {
// for (Command command : commands) {
// command.execute(this.position);
// }
// }
//
// public String get_position_info() {
// return position.toString();
// }
// }
