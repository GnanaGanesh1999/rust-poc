use mars_rover::{Rover, Direction, RoverHandler, MOVE, LEFT, RIGHT};

#[test]
fn test() {
    let rovers = vec![
        Rover::new(0, 0, Direction::North),
        Rover::new(1, 1, Direction::South),
    ];
    let command_lists = vec![
        vec![MOVE, MOVE, LEFT],
        vec![LEFT, LEFT, RIGHT, MOVE],
    ];
    let mut mars_rover = RoverHandler::new(command_lists, rovers);
    let expected_result = String::from("0, 2 : West\n2, 1 : East\n");

    let actual_result = mars_rover.run();

    assert_eq!(expected_result, actual_result);
}

