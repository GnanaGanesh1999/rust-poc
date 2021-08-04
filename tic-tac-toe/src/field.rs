use crate::game_util;

pub struct Field {
    field: [char; 9],
}

impl Field {
    pub fn new() -> Self {
        let field = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        Self { field }
    }

    pub fn is_winner_exist(&self) -> bool {
        for j in 0..=15 {
            let mut coincidences_number = 0;
            for i in 0..9 {
                if self.field[i] == game_util::WINING_COMBINATIONS[j][i] &&
                    ('X' == game_util::WINING_COMBINATIONS[j][i] ||
                        'O' == game_util::WINING_COMBINATIONS[j][i]) {
                    coincidences_number += 1;
                }
                if coincidences_number == 3 {
                    return true;
                }
            }
        }
        false
    }

    pub fn update_field(&mut self, cell_number: usize, value: char) {
        self.field[cell_number - 1] = value;
    }

    pub fn is_cell_free(&self, cell_number: usize) -> bool {
        self.field[cell_number - 1] != 'X' && self.field[cell_number - 1] != 'O'
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        let mut out = String::from("\n");
        let field = self.field;
        out.push_str((field[0].to_string() + " ").as_str());
        out.push_str((field[1].to_string() + " ").as_str());
        out.push_str((field[2].to_string() + " \n").as_str());
        out.push_str((field[3].to_string() + " ").as_str());
        out.push_str((field[4].to_string() + " ").as_str());
        out.push_str((field[5].to_string() + " \n").as_str());
        out.push_str((field[6].to_string() + " ").as_str());
        out.push_str((field[7].to_string() + " ").as_str());
        out.push_str((field[8].to_string() + " \n").as_str());
        out
    }
}
