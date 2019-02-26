use std::fmt;
use rand::prelude::*;

#[derive(Debug)]
pub enum GameState {
    WIN_X,
    WIN_O,
    CAT,
    TURN_X,
    TURN_O,
}

pub struct TicTTGame {
    pub user: String,
    pub x_char: String,
    pub o_char: String,
    board: Vec<Option<String>>,
    pub state: GameState,
    x_sum: u8,
    y_sum: u8,
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
            state: if rand::random() { GameState::TURN_X } else { GameState::TURN_O },
            x_char: x_character,
            o_char: o_character,
            x_sum: 0,
            y_sum: 0,
            n_moves: 0,
        }   
    }

    pub fn update_board(&mut self, x: String, y: String) -> Result<(), &'static str> {  
        let (index, value) = match TicTTGame::equiv_index((&x, &y)) {
            (Some(dex), Some(val)) => (dex, val),
            (_, _) => return Err("Invalid position"),
        };

       let piece: &String = match self.board[index] {
            Some(_) => return Err("That position is already occupied"),
            None => {
                let ret = match &self.state {
                    TURN_X => {
                        self.x_sum += value;
                        &self.x_char
                    },
                    TURN_O => {
                        self.y_sum += value;
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

       self.board[index] = Some(piece.to_string());
       Ok(())
    } 

    pub fn determine_state(game: &TicTTGame) -> GameState {
        if let Some(winner) = game.detect_win() {
            return winner;
        }

        if (game.n_moves == 17) {
            return GameState::CAT;
        }

        match &game.state {
            TURN_X => GameState::TURN_O,
            TURN_O => GameState::TURN_X,
        }
    }

    pub fn equiv_index((x, y): (&str, &str)) -> (Option<usize>, Option<u8>) {
        match ((x, y)) {
            ("0", "0") => (Some(0), Some(8)),
            ("0", "1") => (Some(3), Some(3)),
            ("0", "2") => (Some(6), Some(4)),
            ("1", "0") => (Some(1), Some(1)),
            ("1", "1") => (Some(4), Some(5)),
            ("1", "2") => (Some(7), Some(9)),
            ("2", "0") => (Some(2), Some(6)),
            ("2", "1") => (Some(5), Some(7)), 
            ("2", "2") => (Some(8), Some(2)),
            (_, _) => (None, None),
        }
    }
    
    pub fn detect_win(&self) -> Option<GameState> {
        if self.x_sum == 15 {
            Some(GameState::WIN_X)
        } else if self.y_sum == 15 {
            Some(GameState::WIN_O)
        } else {
            None
        }
    }
}

