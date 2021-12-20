pub const HELLO_MSG: &str = "Hello! Let's start TicTacToe game!";
pub const INCORRECT_CELL_NUMBER_MSG: &str = "This cell number is incorrect. Please, try again.";
pub const USER_MOVE_MSG: &str = "Your move! Choose cell's number ...";
pub const SYSTEM_MOVE_MSG: &str = "The system move ...";
pub const USER_WINNER_MSG: &str = "You are winner!";
pub const SYSTEM_WINNER_MSG: &str = "The system is winner!";
pub const NO_WINNER_MSG: &str = "No winner ...";

pub enum Player {
    USER,
    SYSTEM,
}

pub const WINING_COMBINATIONS: [[char; 9]; 16] = [
    ['X', ' ', ' ', 'X', ' ', ' ', 'X', ' ', ' '],
    ['X', ' ', ' ', ' ', 'X', ' ', ' ', ' ', 'X'],
    ['X', 'X', 'X', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', 'X', 'X', 'X', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', 'X', 'X', 'X'],
    [' ', ' ', 'X', ' ', 'X', ' ', 'X', ' ', ' '],
    [' ', 'X', ' ', ' ', 'X', ' ', ' ', 'X', ' '],
    [' ', ' ', 'X', ' ', ' ', 'X', ' ', ' ', 'X'],
    ['O', ' ', ' ', 'O', ' ', ' ', 'O', ' ', ' '],
    ['O', ' ', ' ', ' ', 'O', ' ', ' ', ' ', 'O'],
    ['O', 'O', 'O', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', 'O', 'O', 'O', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', 'O', 'O', 'O'],
    [' ', ' ', 'O', ' ', 'O', ' ', 'O', ' ', ' '],
    [' ', 'O', ' ', ' ', 'O', ' ', ' ', 'O', ' '],
    [' ', ' ', 'O', ' ', ' ', 'O', ' ', ' ', 'O'],
];
