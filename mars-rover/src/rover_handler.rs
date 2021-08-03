use crate::command::Command;
use crate::rover::Rover;

pub struct RoverHandler {
    command_lists: Vec<Vec<Command>>,
    rovers: Vec<Rover>,
}

impl RoverHandler {
    pub fn new(command_lists: Vec<Vec<Command>>, rovers: Vec<Rover>) -> Self {
        Self { rovers, command_lists }
    }

    pub fn run(&mut self) -> String {
        let mut out = String::new();
        let mut count: usize = 0;
        let number_of_rovers = self.command_lists.len();

        while count < number_of_rovers as usize {
            let rover: &mut Rover = self.rovers.get_mut(count).unwrap();
            let command_list = self.command_lists.get(count).unwrap();

            rover.execute_commands(command_list);
            out += &rover.to_string();
            count += 1;
        }
        out
    }
}

// public String run(String input) {
// StringBuilder out = new StringBuilder();
//
// int numberOfRovers = parseInput(input);
//
// for (int i = 0; i < numberOfRovers; i++) {
// Rover rover = rovers.get(i);
// rover.execute_commands(commandLists.get(i));
// out.append(rover.get_position_info());
// }
//
// return out.toString();
// }