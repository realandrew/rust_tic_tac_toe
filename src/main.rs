use std::fmt;
use rand::Rng;

#[derive(PartialEq)]
enum BoardSpace {
    Empty,
    X,
    O,
}

impl fmt::Debug for BoardSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BoardSpace::Empty => write!(f, "#"),
            BoardSpace::X => write!(f, "X"),
            BoardSpace::O => write!(f, "O"),
        }
    }
}

impl fmt::Display for BoardSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BoardSpace::Empty => write!(f, "#"),
            BoardSpace::X => write!(f, "X"),
            BoardSpace::O => write!(f, "O"),
        }
    }
}

enum GameState {
    XTurn,
    OTurn,
    XWin,
    OWin,
    Tie,
}

enum Player {
    X,
    O,
}

struct Game {
    board: [BoardSpace; 9],
    state: GameState,
    human_player: Player,
    computer_player: Player,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [
                BoardSpace::Empty,
                BoardSpace::Empty,
                BoardSpace::Empty,
                BoardSpace::Empty,
                BoardSpace::Empty,
                BoardSpace::Empty,
                BoardSpace::Empty,
                BoardSpace::Empty,
                BoardSpace::Empty,
            ], // [BoardSpace::Empty; 9] is shorter once BoardSpace gets the Copy trait
            state: GameState::XTurn,
            human_player: Player::X,
            computer_player: Player::O,
        }
    }

    fn play(&mut self) {
        println!("Do you want to play X or O?");
        let mut player_input = String::new();
        std::io::stdin().read_line(&mut player_input).expect("Failed to read line");
        match player_input.trim() {
            "X" => {
                self.human_player = Player::X;
                self.computer_player = Player::O;
            },
            "x" => {
                self.human_player = Player::X;
                self.computer_player = Player::O;
            },
            "O" => {
                self.human_player = Player::O;
                self.computer_player = Player::X;
            },
            "o" => {
                self.human_player = Player::O;
                self.computer_player = Player::X;
            },
            "0" => {
                self.human_player = Player::O;
                self.computer_player = Player::X;
            },
            _ => panic!("Invalid player"),
        };
        println!("Flipping a coin to decide who goes first...");
        let mut rng = rand::thread_rng();
        let coin_flip = rng.gen_range(0..2);
        println!("Coin flip: {}", match coin_flip {
            0 => {
                self.state = GameState::XTurn;
                if let Player::X = self.human_player {
                    self.print()
                }
                "Heads (X goes first)"
            },
            1 => {
                self.state = GameState::OTurn;
                if let Player::O = self.human_player {
                    self.print()
                }
                "Tails (O goes first)"
            },
            _ => "Error",
        });
        self.do_turn();
    }

    fn print(&self) {
        println!("{} | {} | {}", &self.board[0], &self.board[1], &self.board[2]);
        println!("{}", "---------");
        println!("{} | {} | {}", &self.board[3], &self.board[4], &self.board[5]);
        println!("{}", "---------");
        println!("{} | {} | {}", &self.board[6], &self.board[7], &self.board[8]);
    }

    fn set_spot(&mut self, spot: u8, player: &Player) {
        self.board[spot as usize] = match player {
            Player::X => BoardSpace::X,
            Player::O => BoardSpace::O,
        };
    }

    fn do_human_turn(&mut self) {
        println!("Please enter a number between 1 and 9 to place your piece.");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut input: u8 = input.trim().parse().expect("Please type a number!");
        if input < 1 || input > 9 {
            println!("Your number must be between 1 and 9 (so top left is 1, to it's right is 2, etc).");
            self.do_human_turn();
        } else {
            input -= 1;
            match self.human_player {
                Player::X => {
                    self.set_spot(input, &Player::X);
                }
                Player::O => {
                    self.set_spot(input, &Player::O);
                }
            }
        }
    }

    fn do_computer_turn(&mut self) {
        println!("Computer's turn!");
        let mut rng = rand::thread_rng();
        let mut spot = rng.gen_range(0..9);
        while self.board[spot as usize] != BoardSpace::Empty {
            spot = rng.gen_range(0..9);
        }
        match self.computer_player {
            Player::X => {
                self.set_spot(spot, &Player::X);
            }
            Player::O => {
                self.set_spot(spot, &Player::O);
            }
        }
    }

    fn do_turn(&mut self) {
        match self.state {
            GameState::XTurn => {
                match self.human_player {
                    Player::X => {
                        self.do_human_turn();
                    }
                    Player::O => {
                        self.do_computer_turn();
                    }
                }
                self.state = GameState::OTurn;
            }
            GameState::OTurn => {
                match self.human_player {
                    Player::X => {
                        self.do_computer_turn();
                    }
                    Player::O => {
                        self.do_human_turn();
                    }
                }
                self.state = GameState::XTurn;
            }
            _ => {
                println!("Something went wrong!");
            }
        }
        self.print();
        if self.check_win_conditions() {
            self.state = GameState::XWin;
        } else {
            self.do_turn();
        }
    }

    /// Checks if there is a condition of winning.
    /// Returns true if there is a win condition, which includes a tie. Otherwise, returns false.
    /// You can check the condition of winning by looking at the game state (game.state).
    fn check_win_conditions(&mut self) -> bool {
        let player_win = self.check_player_win(&self.human_player);
        let computer_win = self.check_player_win(&self.computer_player);

        if player_win && computer_win {
            self.state = GameState::Tie;
            println!("It's a tie!");
            return true;
        } else if player_win {
            match self.human_player {
                Player::X => {
                    self.state = GameState::XWin;
                    println!("You win!");
                }
                Player::O => {
                    self.state = GameState::OWin;
                    println!("You win!");
                }
            }
            return true;
        } else if computer_win {
            match self.computer_player {
                Player::X => {
                    self.state = GameState::XWin;
                    println!("You lose!");
                }
                Player::O => {
                    self.state = GameState::OWin;
                    println!("You lose!");
                }
            }
            return true;
        } else {
            return false;
        }
    }

    fn check_player_win(&self, player: &Player) -> bool {
        let space_to_check = match player {
            Player::X => BoardSpace::X,
            Player::O => BoardSpace::O,
        };
        if self.check_row(&space_to_check, 0) || self.check_row(&space_to_check, 1) || self.check_row(&space_to_check, 2) {
            return true;
        } else if self.check_col(&space_to_check, 0) || self.check_col(&space_to_check, 1) || self.check_col(&space_to_check, 2) {
            return true;
        } else if self.check_diag(&space_to_check) {
            return true;
        } else {
            return false;
        }
    }

    fn check_row(&self, space_type: &BoardSpace, row : usize) -> bool {
        if self.board[row * 3] == *space_type && self.board[(row * 3) + 1] == *space_type && self.board[(row * 3) + 2] == *space_type {
            return true;
        } else {
            return false;
        }
    }

    fn check_col(&self, space_type: &BoardSpace, col : usize) -> bool {
        if self.board[col] == *space_type && self.board[col + 3] == *space_type && self.board[col + 6] == *space_type {
            return true;
        } else {
            return false;
        }
    }

    fn check_diag(&self, space_type: &BoardSpace) -> bool {
        if self.board[0] == *space_type && self.board[4] == *space_type && self.board[8] == *space_type {
            return true;
        } else if self.board[2] == *space_type && self.board[4] == *space_type && self.board[6] == *space_type {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.play();
}