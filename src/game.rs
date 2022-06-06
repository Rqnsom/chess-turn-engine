pub mod availableturn;
mod board;
mod board_map;
mod builder;
mod castling_rights;
mod castlinginfo;
mod display;
mod enpassant;
pub mod game_error;
pub mod gamestate;
mod king;
mod movement;
mod play;
mod side;
mod simulation;
mod state;

use super::DisplayOption;
use availableturn::AvailableTurn;
use board::Board;
use chess_notation_parser::Piece;
use game_error::GameError;
use gamestate::Gamestate;
use side::Side;
use state::State;

/// Default chessboard setup
pub const NORMAL_SETUP: &'static str = "
    a1,w,R b1,w,N c1,w,B d1,w,Q e1,w,K f1,w,B g1,w,N h1,w,R \
    a2,w,P b2,w,P c2,w,P d2,w,P e2,w,P f2,w,P g2,w,P h2,w,P \
    a7,b,P b7,b,P c7,b,P d7,b,P e7,b,P f7,b,P g7,b,P h7,b,P \
    a8,b,R b8,b,N c8,b,B d8,b,Q e8,b,K f8,b,B g8,b,N h8,b,R
";

/// Chess game
pub struct Game {
    /// Turn history in vector of `State`s.
    history: Vec<State>,

    /// Board contains all the info about current game
    board: Board,

    /// List of available turns
    ///
    /// It's calculated after each turn is played
    available_turns: Vec<AvailableTurn>,
}

impl Game {
    /// Create `Game` instance
    pub fn new(setup: &'static str) -> Result<Game, &'static str> {
        Ok(builder::create(setup)?)
    }

    /// Prepare a string that displays the board
    pub fn display(&self, display_opt: DisplayOption) -> String {
        display::display_game(&self, display_opt)
    }

    /// Get game status
    pub fn gamestate(&self) -> Gamestate {
        self.board.gamestate.clone()
    }

    /// Play turn and update the board status.
    /// Returns old state of the board.
    pub fn play_turn(&mut self, turn: &str) -> Result<Gamestate, GameError> {
        self.history.push(self.board.next_turn(turn)?);

        if self.board.gamestate != Gamestate::Ongoing {
            return Ok(self.board.gamestate.clone());
        }

        // For an ongoing game update available turns
        self.available_turns = simulation::get_available_turns(&mut self.board);

        // Some draw conditions are set in `board.next_turn` function
        self.check_few_draw_conditions();

        if self.board.gamestate != Gamestate::Ongoing {
            self.available_turns = vec![];
        }

        Ok(self.board.gamestate.clone())
    }

    /// Undo turn and restore board state
    pub fn undo_turn(&mut self) -> Result<(), GameError> {
        let prev_state = match self.history.pop() {
            None => return Err(GameError::UndoNotAvailable),
            Some(state) => state,
        };

        self.board.active_player.switch_side();
        // Switch player before calculating hash! Anything else is a headache!
        self.board.hash_state_pop();
        self.board.undo(prev_state);

        self.available_turns = simulation::get_available_turns(&mut self.board);
        Ok(())
    }

    /// Get list of available turns
    pub fn available_turns(&self) -> &Vec<AvailableTurn> {
        &self.available_turns
    }

    /// Update gamestate for the draw conditions
    fn check_few_draw_conditions(&mut self) {
        if self.available_turns.is_empty() {
            self.board.gamestate = Gamestate::Stealmate;
            return;
        }

        // Mating is not possible in the following conditions:
        // -> K vs K    (len must be 2 for this condition)
        // -> K+B vs K
        // -> K+N vs K
        if self.board.map.len() == 2
            || (self.board.map.len() == 3
                && !self
                    .board
                    .map
                    .into_iter()
                    .map(|(_, (p, _))| p)
                    .filter(|p| *p == Piece::Knight || *p == Piece::Bishop)
                    .collect::<Vec<Piece>>()
                    .is_empty())
        {
            self.board.gamestate = Gamestate::DrawInsufficientMatingMaterial;
        }
    }
}
