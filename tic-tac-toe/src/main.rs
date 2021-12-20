use crate::tic_tac_toe::TicTacToe;

mod field;
mod game_util;
mod tic_tac_toe;

fn main() {
    let mut gl = TicTacToe::new();
    gl.start();
}
