use crate::tic_tac_toe::TicTacToe;

mod game_util;
mod field;
mod tic_tac_toe;

fn main() {
    let mut gl = TicTacToe::new();
    gl.start();
}
