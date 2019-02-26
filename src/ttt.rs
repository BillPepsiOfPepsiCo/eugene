use std::fmt;
use rand::prelude::*;

#[derive(Debug)]
pub enum GameState {
    win_x,
    win_o,
    cat,
    turn_x,
    turn_o,
}

pub struct TicTTGame {
    pub user: String,
    pub x_char: String,
    pub o_char: String,
    board: Vec<Option<String>>,
    pub state: GameState,
    x_sum: u8,
    o_sum: u8,
    n_moves: u8,
}

impl fmt::Display for TicTTGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp_str: String = String::new();
        disp_str.push_str(&format!("TicTTGame: [ User: {} State: {:?} X_Char: {} O_Char: {} ]\n", self.user, self.state, self.x_char, self.o_char));
        disp_str.push_str("Board:\n");
        disp_str.push_str(&format!("{:?}\n", self.board));
        write!(f, "{}", disp_str)
    }
}

impl TicTTGame {
    pub fn new(user: String, x_character: String, o_character: String) -> TicTTGame {
        TicTTGame {
            user: user,
            board: vec![None, None, None, None, None, None, None, None, None],
            state: if rand::random() { GameState::turn_x } else { GameState::turn_o },
            x_char: x_character,
            o_char: o_character,
            x_sum: 0,
            o_sum: 0,
            n_moves: 0,
        }   
    }

    pub fn update_board(&mut self, pos: String) -> Result<(), &'static str> {  
        let position = match pos.parse::<u8>() {
            Ok(val) => val as usize,
            Err(why) => return Err("Your input is either too big or non-numeric!"),
        };

        let ms_val = match TicTTGame::getms_value(position as u8) {
            Some(val) => val,
            None => return Err("Invalid position!"),
        };

       let piece: &String = match self.board[position] {
            Some(_) => return Err("That position is already occupied"),
            None => {
                let ret = match &self.state {
                    turn_x => {
                        self.x_sum += ms_val;
                        &self.x_char
                    },
                    turn_o => {
                        self.o_sum += ms_val;
                        &self.o_char
                    },
                    _ => return Err("Game is unable to be updated: invalid state"),
                };

                //Update the state for the next turn
                self.n_moves += 1;
                self.state = TicTTGame::determine_state(&self);
                ret
            },
        };

       self.board[position] = Some(piece.to_string());
       Ok(())
    } 

    //This function gives the corresponding magic square
    //value for whatever position the user wants
    pub fn getms_value(location: u8) -> Option<u8> {
        match location {
            0 => Some(8),
            1 => Some(1),
            2 => Some(6),
            3 => Some(3),
            4 => Some(5),
            5 => Some(7),
            6 => Some(4),
            7 => Some(9),
            8 => Some(2),
            _ => None,
        }
    }

    pub fn determine_state(game: &TicTTGame) -> GameState {
        if let Some(winner) = game.detect_win() {
            return winner;
        }

        if (game.n_moves == 17) {
            return GameState::cat;
        }

        match &game.state {
            turn_x => GameState::turn_o,
            turn_o => GameState::turn_x,
        }
    }

    pub fn detect_win(&self) -> Option<GameState> {
        if self.x_sum == 15 {
            Some(GameState::win_x)
        } else if self.o_sum == 15 {
            Some(GameState::win_o)
        } else {
            None
        }
    }

    pub fn board_grid() -> &'static str {
        "0 | 1 | 2\n
        -----------\n
         3 | 4 | 5\n
        -----------\n
         6 | 7 | 8\n"
    }
}

