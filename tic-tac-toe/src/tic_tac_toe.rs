use crate::field::Field;
use crate::game_util;
use crate::game_util::Player;
use rand::Rng;
use std::io::{stdin, Error};

pub struct TicTacToe {
    field: Field,
    move_number: u8,
    current_cell_number: u8,
}

impl TicTacToe {
    pub(crate) fn new() -> Self {
        let field = Field::new();
        let current_player = game_util::Player::USER;
        Self {
            field,
            move_number: 0,
            current_cell_number: 1,
        }
    }

    pub(crate) fn start(&mut self) {
        println!("{}", self.field.to_string());
        while self.move_number < 9 {
            println!("{}", game_util::USER_MOVE_MSG);
            self.user_move();
            self.field
                .update_field(self.current_cell_number as usize, 'X');
            self.move_number += 1;
            println!("{}", self.field.to_string());

            // self.field.update_field(self.current_cell_number, 'O');

            if self.field.is_winner_exist() {
                println!("{}", game_util::USER_WINNER_MSG);
                break;
            } else if self.move_number < 9 {
                println!("{}", game_util::SYSTEM_MOVE_MSG);
                self.system_move();
            }
            self.field
                .update_field(self.current_cell_number as usize, 'O');
            self.move_number += 1;
            println!("{}", self.field.to_string());

            if self.field.is_winner_exist() {
                println!("{}", game_util::SYSTEM_WINNER_MSG);
                break;
            }
        }
        if !self.field.is_winner_exist() {
            println!("{}", game_util::NO_WINNER_MSG);
        }
    }

    fn system_move(&mut self) {
        let system_cell_number: u8 = rand::thread_rng().gen_range(1..10);
        self.current_cell_number = system_cell_number;
        if !self.is_valid_cell(system_cell_number) {
            self.system_move()
        }
    }

    fn user_move(&mut self) {
        let mut cell_number_str = String::new();
        stdin().read_line(&mut cell_number_str).unwrap();
        let cell_number_str_argument = &mut cell_number_str;
        match cell_number_str_argument.trim().parse() {
            Ok(x) => {
                self.current_cell_number = x;
                if !self.is_valid_cell(x) {
                    self.user_move();
                }
            }
            Err(_) => {
                println!("{}", game_util::INCORRECT_CELL_NUMBER_MSG);
                self.user_move();
            }
        };
    }

    fn is_valid_cell(&self, cell_number: u8) -> bool {
        cell_number < 10 && cell_number > 0 && self.field.is_cell_free(cell_number as usize)
    }
}
