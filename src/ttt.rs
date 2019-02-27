use std::fmt;
use rand::prelude::*;

#[derive(Debug)]
pub enum GameState {
    Win_Player1,
    Win_Player2,
    cat,
    Turn_Player1,
    Turn_Player2,
}

#[derive(Debug)]
pub struct TicTTGame {
    pub player1: Player,
    pub player2: Player,
    pub state: GameState,
    board: Vec<Option<String>>,
    total_moves: u8,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub piece: String,
    pub points: u8,
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.name == other.name
    }
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

impl Player {
    pub fn new(name: String, piece: String) -> Player {
        Player {
            name: name,
            piece: piece,
            points: 0,
        }
    }
}

impl TicTTGame {
    pub fn new(player1: Player, player2: Player) -> TicTTGame {
        TicTTGame {
            player1: player1,
            player2: player2,
            board: vec![None, None, None, None, None, None, None, None, None],
            state: if rand::random() { GameState::Turn_Player1 } else { GameState::Turn_Player2 },
            total_moves: 0,
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
            Some(_) => return Err("That position is already occupied!"),
            None => {
                let ret = match &self.state {
                    Turn_Player1 => {
                        self.player1.points += ms_val;
                        &self.player1.piece
                    },
                    Turn_Player2 => {
                        self.player2.points += ms_val;
                        &self.player2.piece
                    },
                    _ => return Err("Game is unable to be updated: invalid state"),
                };

                //Update the state for the next turn
                self.total_moves += 1;
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

        if (game.total_moves == 17) {
            return GameState::cat;
        }

        match &game.state {
            Turn_Player1 => GameState::Turn_Player2,
            Turn_Player2 => GameState::Turn_Player1,
        }
    }

    pub fn detect_win(&self) -> Option<GameState> {
        if self.player1.points == 15 {
            Some(GameState::Win_Player1)
        } else if self.player2.points == 15 {
            Some(GameState::Win_Player2)
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

