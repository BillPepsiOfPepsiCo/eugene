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

#[derive(Debug)]
pub struct TicTTGame {
    pub player1: String,
    pub player2: String,
    pub x_char: String,
    pub o_char: String,
    pub state: GameState,
    board: Vec<Option<String>>,
    x_sum: u8,
    o_sum: u8,
    n_moves: u8,
}

impl fmt::Display for TicTTGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp_str: String = String::new();
        let mut clean_board: Vec<String> = Vec::new();

        for thing in &self.board {
            match thing {
                Some(piece) => clean_board.push(piece.to_string()),
                None => clean_board.push(String::from("-")),
            };
        }
        
        disp_str.push_str(&format!("{} | {} | {}\n", clean_board[0], clean_board[1], clean_board[2]));
        disp_str.push_str("----------\n");
        disp_str.push_str(&format!("{} | {} | {}\n", clean_board[3], clean_board[4], clean_board[5])); 
        disp_str.push_str("----------\n");
        disp_str.push_str(&format!("{} | {} | {}\n", clean_board[6], clean_board[7], clean_board[8]));

        write!(f, "{}", disp_str)
    }
}

impl TicTTGame {
    pub fn new(player1: String, x_char: String, o_char: String) -> TicTTGame {
        TicTTGame {
            player1: String::new(),
            player2: String::new(),
            board: vec![None, None, None, None, None, None, None, None, None],
            state: if rand::random() { GameState::turn_x } else { GameState::turn_o },
            x_char: String::new(),
            o_char: String::new(),
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

