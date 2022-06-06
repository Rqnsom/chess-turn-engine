//! # Chess turn engine
//! Turn engine fully implements all chess rules so it could be used to play
//! chess or to make chess simulations.
//!
//! Engine provides a list of available and playable turns after each turn
//! is played. Any turn from the list can be played.
//! Any played turn can be undone.
//!
//! Input for the engine are chess turns written in algebraic chess notation
//! format. But for the simplicity sake, one can use `AvailableTurn` struct to
//! transform simple layman's turn *Move piece from square A to square B* into a
//! chess notation turn.
//!
//! ## Display
//! Chess engine provides a basic set of tools to print out the board state in
//! ASCII format. Few options are available for printing out the board state.
//! Turn history can be printed out. Captured pieces can be printed out.
//! ### `ViewMode::SimpleAscii` - Quite simple display format
//! #### Legend
//!    - `b`: Black player
//!    - `w`: White player
//!    - `P`: Pawn
//!    - `N`: Knight
//!    - `B`: Bishop
//!    - `R`: Rook
//!    - `K`: King
//!    - `Q`: Queen
//!
//!  #### Example
//!  ```text
//!  8 bR bN bB bQ bK bB bN bR
//!  7 bP bP bP bP bP bP bP bP
//!  6  -  +  -  +  -  +  -  +
//!  5  +  -  +  -  +  -  +  -
//!  4  -  +  -  +  -  +  -  +
//!  3  +  -  +  -  +  -  +  -
//!  2 wP wP wP wP wP wP wP wP
//!  1 wR wN wB wQ wK wB wN wR
//!     a  b  c  d  e  f  g  h
//!  ```
//! ### `ViewMode::FancyTui` - Colorful terminal ASCII display format
//! Note: *colors are not visible in documentation pages*
//!
//! #### Example
//! ```text
//! 8  ♜  ♞  ♝  ♛  ♚  ♝  ♞  ♜
//! 7  ♟  ♟  ♟  ♟  ♟  ♟  ♟  ♟
//! 6
//! 5
//! 4
//! 3
//! 2  ♙  ♙  ♙  ♙  ♙  ♙  ♙  ♙
//! 1  ♖  ♘  ♗  ♕  ♔  ♗  ♘  ♖
//!    a  b  c  d  e  f  g  h
//! ```
//! ## Example with six straightforward turns played
//! ```
//! # use chess_turn_engine::{
//! #     ChessTurnEngine, DisplayOption, Setup, ViewMode, Gamestate
//! # };
//! #
//! let mut cte = ChessTurnEngine::new(Setup::Normal).unwrap();
//! let turns = ["h4", "Na6", "h5", "b5", "d3", "g6"];
//!
//! turns.iter().for_each(|turn| {
//!     cte.play_turn(turn);
//! });
//!
//! for turn in cte.available_turns() {
//!     println!("{}", turn);
//! }
//!
//! // Output:
//! // AvailableTurn (src: h5, dst: h6, piece: Pawn, captured: None, turn: h6
//! // AvailableTurn (src: h5, dst: g6, piece: Pawn, captured: Some("Pawn"), turn: hxg6
//! // AvailableTurn (src: d3, dst: d4, piece: Pawn, captured: None, turn: d4
//! // AvailableTurn (src: a2, dst: a3, piece: Pawn, captured: None, turn: a3
//! // AvailableTurn (src: a2, dst: a4, piece: Pawn, captured: None, turn: a4
//! // AvailableTurn (src: b2, dst: b3, piece: Pawn, captured: None, turn: b3
//! // AvailableTurn (src: b2, dst: b4, piece: Pawn, captured: None, turn: b4
//! // AvailableTurn (src: c2, dst: c3, piece: Pawn, captured: None, turn: c3
//! // AvailableTurn (src: c2, dst: c4, piece: Pawn, captured: None, turn: c4
//! // AvailableTurn (src: e2, dst: e3, piece: Pawn, captured: None, turn: e3
//! // AvailableTurn (src: e2, dst: e4, piece: Pawn, captured: None, turn: e4
//! // AvailableTurn (src: f2, dst: f3, piece: Pawn, captured: None, turn: f3
//! // AvailableTurn (src: f2, dst: f4, piece: Pawn, captured: None, turn: f4
//! // AvailableTurn (src: g2, dst: g3, piece: Pawn, captured: None, turn: g3
//! // AvailableTurn (src: g2, dst: g4, piece: Pawn, captured: None, turn: g4
//! // AvailableTurn (src: b1, dst: c3, piece: Knight, captured: None, turn: Nc3
//! // AvailableTurn (src: b1, dst: a3, piece: Knight, captured: None, turn: Na3
//! // AvailableTurn (src: b1, dst: d2, piece: Knight, captured: None, turn: Nd2
//! // AvailableTurn (src: c1, dst: d2, piece: Bishop, captured: None, turn: Bd2
//! // AvailableTurn (src: c1, dst: e3, piece: Bishop, captured: None, turn: Be3
//! // AvailableTurn (src: c1, dst: f4, piece: Bishop, captured: None, turn: Bf4
//! // AvailableTurn (src: c1, dst: g5, piece: Bishop, captured: None, turn: Bg5
//! // AvailableTurn (src: c1, dst: h6, piece: Bishop, captured: None, turn: Bh6
//! // AvailableTurn (src: d1, dst: d2, piece: Queen, captured: None, turn: Qd2
//! // AvailableTurn (src: e1, dst: d2, piece: King, captured: None, turn: Kd2
//! // AvailableTurn (src: g1, dst: h3, piece: Knight, captured: None, turn: Nh3
//! // AvailableTurn (src: g1, dst: f3, piece: Knight, captured: None, turn: Nf3
//! // AvailableTurn (src: h1, dst: h2, piece: Rook, captured: None, turn: Rh2
//! // AvailableTurn (src: h1, dst: h3, piece: Rook, captured: None, turn: Rh3
//! // AvailableTurn (src: h1, dst: h4, piece: Rook, captured: None, turn: Rh4
//! ```
//! ## Example with two turns played
//! ```
//! # use chess_turn_engine::{
//! #     ChessTurnEngine, DisplayOption, Setup, ViewMode, Gamestate
//! # };
//! #
//! let mut cte = ChessTurnEngine::new(Setup::Normal).unwrap();
//! assert!(cte.play_turn("d4").is_ok()); // Play "d4", starting turn
//!
//! // Play a random turn for the black player
//! let mut next_random_turn =
//!     String::from(cte.available_turns().first().unwrap().get_turn());
//! assert!(cte.play_turn(&next_random_turn).is_ok());
//!
//! // Play "a3" turn using only info about the source and destination squares
//! for turn in cte.available_turns() {
//!     if turn.src != String::from("a2") && turn.dst != String::from("a3") {
//!         continue;
//!     }
//!     next_random_turn = String::from(turn.get_turn());
//!     break;
//! }
//! assert!(cte.play_turn(&next_random_turn).is_ok());
//! assert_eq!(cte.gamestate(), Gamestate::Ongoing);
//!
//! cte.display_on_screen(DisplayOption::BoardView(ViewMode::SimpleAscii));
//! // 8 bR bN bB bQ bK bB bN bR
//! // 7 bP bP bP  - bP bP bP bP
//! // 6  -  +  -  +  -  +  -  +
//! // 5  +  -  + bP  +  -  +  -
//! // 4  -  +  - wP  -  +  -  +
//! // 3 wP  -  +  -  +  -  +  -
//! // 2  - wP wP  + wP wP wP wP
//! // 1 wR wN wB wQ wK wB wN wR
//! //    a  b  c  d  e  f  g  h
//! ```

mod game;

pub use game::availableturn::AvailableTurn;
pub use game::game_error::GameError;
pub use game::gamestate::Gamestate;
use game::Game;

/// Game engine
pub struct ChessTurnEngine {
    /// Game
    game: Game,
}

/// View modes for `BoardView`
#[derive(Copy, Clone)]
pub enum ViewMode {
    /// Display board in plain ASCII format
    SimpleAscii,

    /// Display board in colorful ASCII format for terminal purposes
    FancyTui,
}

/// Display option
#[derive(Copy, Clone)]
pub enum DisplayOption {
    /// Display board state
    BoardView(ViewMode),

    /// Display turn history in plain ASCII format
    TurnHistory,

    /// Display capture history in plain ASCII format
    CaptureHistory,
}

/// Chessboard setup
#[derive(Copy, Clone)]
pub enum Setup {
    /// **Default** board setup
    Normal,

    /// **Custom setup** (experimental usage)
    ///
    /// Note: *Castling turns are not possible in custom setups*
    ///
    /// Format for the custom setup is:
    ///     ```(`Square`,`Player`,`Piece` )+```
    /// - `Square`
    ///     - Must be specified with lowercase letters: e.g. `a1`
    /// - `Player`
    ///     - `w` for White
    ///     - `b` for Black
    /// - `Piece`
    ///     - `P`: Pawn
    ///     - `R`: Rook
    ///     - `B`: Bishop
    ///     - `N`: Knight
    ///     - `Q`: Queen
    ///     - `K`: King - Note: *game always must have two kings*
    ///
    /// Below is an example of how default chess setup looks like:
    ///
    /// ```
    /// let default_setup =
    ///    "a1,w,R b1,w,N c1,w,B d1,w,Q e1,w,K f1,w,B g1,w,N h1,w,R \
    ///     a2,w,P b2,w,P c2,w,P d2,w,P e2,w,P f2,w,P g2,w,P h2,w,P \
    ///     a7,b,P b7,b,P c7,b,P d7,b,P e7,b,P f7,b,P g7,b,P h7,b,P \
    ///     a8,b,R b8,b,N c8,b,B d8,b,Q e8,b,K f8,b,B g8,b,N h8,b,R";
    ///  ```
    Custom(&'static str),
}

impl ChessTurnEngine {
    /// Create and setup a new game
    ///
    /// # Arguments
    ///
    /// * `setup` - Game setup
    pub fn new(setup: Setup) -> Result<ChessTurnEngine, &'static str> {
        let setup = match setup {
            Setup::Normal => game::NORMAL_SETUP,
            Setup::Custom(setup) => setup,
        };

        let game = Game::new(setup)?;
        Ok(ChessTurnEngine { game })
    }

    /// Provide a displayable `String` of the chessboard
    ///
    /// # Arguments
    ///
    /// * `opt` - Display option
    pub fn display(&self, opt: DisplayOption) -> String {
        self.game.display(opt)
    }

    /// Display chessboard on the screen
    ///
    /// # Arguments
    ///
    /// * `opt` - Display option
    pub fn display_on_screen(&self, opt: DisplayOption) {
        println!("{}", self.game.display(opt));
    }

    /// Play provided turn.
    ///
    /// # Return value
    ///  - `Ok` - Latest game status.
    ///  - `Err` - `GameError` in case turn is not playable.
    ///
    /// # Arguments
    ///
    /// * `turn` - Turn provided in chess notation format
    pub fn play_turn(&mut self, turn: &str) -> Result<Gamestate, GameError> {
        self.game.play_turn(turn)
    }

    /// Get game status
    pub fn gamestate(&self) -> Gamestate {
        self.game.gamestate()
    }

    /// Undo turn and restore previous board state
    pub fn undo_turn(&mut self) -> Result<(), GameError> {
        Ok(self.game.undo_turn()?)
    }

    /// Get list of available turns
    pub fn available_turns(&self) -> &Vec<AvailableTurn> {
        &self.game.available_turns()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_setup_valid() {
        let cte = ChessTurnEngine::new(Setup::Normal);
        assert!(cte.is_ok());

        let cte = cte.unwrap();
        // Make sure turn and capture histry are empty
        assert_eq!(cte.display(DisplayOption::TurnHistory), String::new());
        assert_eq!(cte.display(DisplayOption::CaptureHistory), String::new());

        cte.display_on_screen(DisplayOption::BoardView(ViewMode::FancyTui));
        cte.display_on_screen(DisplayOption::BoardView(ViewMode::SimpleAscii));

        // Same effect as above
        println!(
            "{}",
            cte.display(DisplayOption::BoardView(ViewMode::FancyTui))
        );
        println!(
            "{}",
            cte.display(DisplayOption::BoardView(ViewMode::SimpleAscii))
        );
    }

    #[test]
    fn custom_setup_valid() {
        const NO_PAWNS: &'static str = "
            a1,w,R b1,w,N c1,w,B d1,w,Q e1,w,K f1,w,B g1,w,N h1,w,R \
            a8,b,R b8,b,N c8,b,B d8,b,Q e8,b,K f8,b,B g8,b,N h8,b,R
        ";

        let cte = ChessTurnEngine::new(Setup::Custom(NO_PAWNS));
        assert!(cte.is_ok());

        let cte = cte.unwrap();
        cte.display_on_screen(DisplayOption::BoardView(ViewMode::FancyTui));
        cte.display_on_screen(DisplayOption::BoardView(ViewMode::SimpleAscii));
    }

    #[test]
    fn custom_setup_invalid_format() {
        const GARBAGE: &'static str = "!aaa.1122";
        assert!(ChessTurnEngine::new(Setup::Custom(GARBAGE)).is_err());

        const MISSING_COMMA: &'static str = "a1,w,R w,N e1,w,K e8,b,K";
        assert!(ChessTurnEngine::new(Setup::Custom(MISSING_COMMA)).is_err());

        const MISSING_COMMA1: &'static str = ",a1wR w,N e1,w,K e8,b,K";
        assert!(ChessTurnEngine::new(Setup::Custom(MISSING_COMMA1)).is_err());

        const EXTRA_COMMA: &'static str = "a1,w,R a1,w,N,N e1,w,K e8,b,K";
        assert!(ChessTurnEngine::new(Setup::Custom(EXTRA_COMMA)).is_err());
    }

    #[test]
    fn custom_setup_with_invalid_square() {
        // xx is not a Square
        const INVALID_SQUARE: &'static str = "a1,w,R xx,w,N e1,w,K e8,b,K";
        assert!(ChessTurnEngine::new(Setup::Custom(INVALID_SQUARE)).is_err());

        // Second str has no Square
        const NO_SQUARE: &'static str = " a1,w,R ,w,N e1,w,K e8,b,K ";
        assert!(ChessTurnEngine::new(Setup::Custom(NO_SQUARE)).is_err());
    }

    #[test]
    fn custom_setup_square_already_taken() {
        // All squares are 'a1'
        const SQUARE_REUSED: &'static str = "a1,w,R a1,w,N a1,w,K a1,b,K";
        assert!(ChessTurnEngine::new(Setup::Custom(SQUARE_REUSED)).is_err());
    }

    #[test]
    fn custom_setup_with_invalid_side() {
        // X is not a Side
        const INVALID_SIDE: &'static str = "c1,w,B e1,X,K a8,b,R e8,b,K";
        assert!(ChessTurnEngine::new(Setup::Custom(INVALID_SIDE)).is_err());

        const NO_SIDE: &'static str = "c1,w,B e1,,K a8,b,R e8,b,K";
        assert!(ChessTurnEngine::new(Setup::Custom(NO_SIDE)).is_err());
    }

    #[test]
    fn custom_setup_with_invalid_piece() {
        // T is not a Piece
        const INVALID_PIECE: &'static str = "c1,w,B e1,w,K a8,b,T e8,b,K";
        assert!(ChessTurnEngine::new(Setup::Custom(INVALID_PIECE)).is_err());

        // Third str is missing a piece
        const NO_PIECE: &'static str = "c1,w,B e1,w,K a8,b, e8,b,K";
        assert!(ChessTurnEngine::new(Setup::Custom(NO_PIECE)).is_err());
    }

    #[test]
    fn custom_setup_with_invalid_number_of_kings() {
        // Four kings is little bit too much
        const MANY_KINGS: &'static str = "c1,w,K e1,w,K a8,b,K e8,b,K";
        assert!(ChessTurnEngine::new(Setup::Custom(MANY_KINGS)).is_err());

        // We need to have exactly two kings
        const NO_KINGS: &'static str = "c1,w,Q e8,b,Q";
        assert!(ChessTurnEngine::new(Setup::Custom(NO_KINGS)).is_err());
    }

    #[test]
    fn cannot_undo_before_the_first_turn_is_played() {
        let mut cte = ChessTurnEngine::new(Setup::Normal).unwrap();

        assert_eq!(cte.undo_turn(), Err(GameError::UndoNotAvailable));
    }

    #[test]
    // Main purpose here is to test that available turn with specific
    // square within the turn notation is generated correctly
    fn custom_setup_with_multiple_knights() {
        const KNIGHTS: &'static str = "
            a1,w,N b1,w,N c1,w,N d1,w,N e1,w,N f1,w,N g1,w,N h1,w,K \
            a8,b,N b8,b,N c8,b,N d8,b,N e8,b,N f8,b,N g8,b,N h8,b,K
        ";

        let mut cte = ChessTurnEngine::new(Setup::Custom(KNIGHTS)).unwrap();
        cte.display_on_screen(DisplayOption::BoardView(ViewMode::FancyTui));

        // Black turns are random, the white guy is doing the testing
        play(&mut cte, "Nab3 Nac7  Nc2 Nb7  Nce2 Na8  Nf3 Nd8");
        play(&mut cte, "Nb1d2"); // Here it works!
    }

    #[test]
    fn test_simulated_castlings() {
        let mut cte = ChessTurnEngine::new(Setup::Normal).unwrap();
        cte.display_on_screen(DisplayOption::BoardView(ViewMode::FancyTui));

        // Black turns are random, the white guy is doing the testing
        play(&mut cte, "f4 e5  fxe5 Qg5  e3 Qxg2  Ba6 b6  Nh3 Qb7");
        play(&mut cte, "e6 Ke7  exd7 Kf6  0-0+");
        undo_turns(&mut cte, 1);

        play(&mut cte, "Nf2 Qxh1+  Nxh1 Ke7");
        // After capturing the rook, short castling is unavailable
        invalid_turn(&mut cte, "0-0", GameError::CastlingUnavailable);

        play(&mut cte, "b3 g5  d8=R Bg7  Na3 Kxd8  Bb2 Bxb2");
        invalid_turn(&mut cte, "0-0-0", GameError::CastlingSquaresNotEmpty);

        play(&mut cte, "Qh5 Bd7");
        invalid_turn(&mut cte, "0-0-0", GameError::KingCannotCastleSafetly);

        play(&mut cte, "Qxf7 Bxa1");
        // After capturing the rook, long castling is unavailable
        invalid_turn(&mut cte, "0-0-0", GameError::CastlingUnavailable);
    }

    #[test]
    fn game_testing_errors1() {
        let mut cte = ChessTurnEngine::new(Setup::Normal).unwrap();

        cte.display_on_screen(DisplayOption::BoardView(ViewMode::FancyTui));

        play(&mut cte, "d3 e6  d4 e5  dxe5  f6 c4  d6 c5  fxe5");

        invalid_turn(&mut cte, "e7", GameError::MovingPieceNotFound);
        invalid_turn(&mut cte, "a5", GameError::MovingPieceNotFound);
        invalid_turn(&mut cte, "ef3", GameError::InvalidPawnMovement);
        invalid_turn(&mut cte, "exf3", GameError::NoCapturePiece);
        invalid_turn(&mut cte, "Nb3", GameError::MovingPieceNotFound);
        invalid_turn(&mut cte, "a3+", GameError::KingIsSafe);
        invalid_turn(&mut cte, "a3#", GameError::KingIsSafe);

        play(&mut cte, "a3 b5  Qd3 Qh4  Nc3 Na6");
        play(&mut cte, "Nxb5 Qa4  Nf3 Qxa3  Rxa3 Bf5");

        invalid_turn(&mut cte, "f7", GameError::MovingPieceNotFound);
        invalid_turn(&mut cte, "Bfg7", GameError::MovingPieceNotFound);
        invalid_turn(&mut cte, "Ra7", GameError::MovingPieceNotFound);
        invalid_turn(&mut cte, "Ra6", GameError::CaptureNotSet);
        invalid_turn(&mut cte, "Ra3", GameError::MovingPieceNotFound);
        invalid_turn(&mut cte, "Qa3", GameError::CaptureNotSet);
        invalid_turn(&mut cte, "Qxa3", GameError::CaptureAlly);
        invalid_turn(&mut cte, "Nxc7", GameError::KingIsInCheck);
        invalid_turn(&mut cte, "Kc3", GameError::MovingPieceNotFound);
        invalid_turn(&mut cte, "Kc2", GameError::MovingPieceNotFound);
        invalid_turn(&mut cte, "g2", GameError::MovingPieceNotFound);
        invalid_turn(&mut cte, "gxh2", GameError::MovingPieceNotFound);
        invalid_turn(&mut cte, "gxh3", GameError::NoCapturePiece);

        play(&mut cte, "cxd6 e4  d7+ Ke7");

        invalid_turn(&mut cte, "d8=B", GameError::KingIsInCheck);
        invalid_turn(&mut cte, "c8=B", GameError::MovingPieceNotFound);
        invalid_turn(&mut cte, "d8=P", GameError::ParsingTurnFailed);
        invalid_turn(&mut cte, "Qd6", GameError::KingIsInCheck);
        invalid_turn(&mut cte, "Qd6#", GameError::KingIsInCheck);

        play(&mut cte, "d8=B+?"); // Black plays next

        invalid_turn(&mut cte, "Kxd8", GameError::OurKingMustBeSafe);
        invalid_turn(&mut cte, "Kf6", GameError::OurKingMustBeSafe);
        invalid_turn(&mut cte, "Kd7", GameError::OurKingMustBeSafe);

        // exf3 is an en-passant turn
        play(&mut cte, "Rxd8  Qxd8+ Kxd8  Rd3+ Bd6  Ne5 Ke8  f4 exf3  e4");

        invalid_turn(&mut cte, "fxe2", GameError::NoCapturePiece);
        invalid_turn(&mut cte, "f2", GameError::KingIsInCheck);

        play(&mut cte, "fxg2  h4 g1=Q!  h5 Qg2  Bxg2 g5  Nxc7+ Bxc7");

        // hxg6 could have been an en-passant a turn earlier but now it's late
        invalid_turn(&mut cte, "hxg6", GameError::NoCapturePiece);
        invalid_turn(&mut cte, "0-0+", GameError::KingIsSafe);
        invalid_turn(&mut cte, "0-0#", GameError::KingIsSafe);
        invalid_turn(&mut cte, "0-0-0", GameError::CastlingUnavailable);
        invalid_turn(&mut cte, "0-0-0+", GameError::CastlingUnavailable);

        play(&mut cte, "Bf1"); // Black plays next

        invalid_turn(&mut cte, "0-0", GameError::CastlingUnavailable);
        invalid_turn(&mut cte, "0-0+", GameError::CastlingUnavailable);
        invalid_turn(&mut cte, "0-0#", GameError::CastlingUnavailable);
        invalid_turn(&mut cte, "0-0-0", GameError::CastlingUnavailable);
        invalid_turn(&mut cte, "0-0-0+", GameError::CastlingUnavailable);

        play(&mut cte, "Nf6"); // White plays next

        invalid_turn(&mut cte, "0-0", GameError::CastlingSquaresNotEmpty);
        invalid_turn(&mut cte, "0-0+", GameError::CastlingSquaresNotEmpty);
        invalid_turn(&mut cte, "0-0#", GameError::CastlingSquaresNotEmpty);

        play(&mut cte, "Bh3 Bxh3");

        invalid_turn(&mut cte, "0-0", GameError::KingCannotCastleSafetly);

        // Let's finish up here by ending in stealmate
        play(&mut cte, "Rdxh3 Ke7  0-0 Rg8  h6 Rg7  hxg7 Nh5  g8=Q g4");
        play(&mut cte, "Rxh5 Bb8  Qxb8 h6  Rxh6 Nb4  Nxg4 a5  Qxb4+ axb4");
        play(&mut cte, "Rff6 Kd7");

        // Special case when both rooks are already on rank 6
        invalid_turn(&mut cte, "R6g6", GameError::MovingPieceNotFound);

        play(&mut cte, "Rf1 Ke7");
        play(&mut cte, "Rb6 b3  Rxb3 Kd7  e5 Kd8  e6 Kc8  e7 Kc7");
        play(&mut cte, "Rf6 Kc8  Bh6 Kc7  e8=Q");

        let gs = cte.gamestate();
        invalid_turn(&mut cte, "d3", GameError::GameOver(gs));
        assert_eq!(Gamestate::Stealmate, cte.gamestate());

        cte.display_on_screen(DisplayOption::BoardView(ViewMode::FancyTui));
    }

    #[test]
    fn game_testing_errors2() {
        let mut cte = ChessTurnEngine::new(Setup::Normal).unwrap();

        play(&mut cte, "d4 e5  Bg5 Qxg5  g4 a6  Bh3 a5  Nf3 exd4");
        play(&mut cte, "e4 a4  Qxd4 Qc1+"); // These will be undone

        // Test case where king is under check and cannot castle
        invalid_turn(&mut cte, "0-0", GameError::CastlingUnderCheck);

        // Let's undo few turns and finish up the game
        undo_turns(&mut cte, 4);
        play(&mut cte, "Qxd4 c5  Qxc5 Qc1#");

        // Test game over
        let gs = cte.gamestate();
        invalid_turn(&mut cte, "a3", GameError::GameOver(gs));
        assert_eq!(Gamestate::Victory("Black".to_owned()), cte.gamestate());

        // Make sure we can undo turn after game is over
        undo_turns(&mut cte, 4);
        assert_eq!(Gamestate::Ongoing, cte.gamestate());
        play(&mut cte, "Qxd4 Qc1+  Qd1");

        invalid_turn(&mut cte, "Bb4", GameError::KingIsInCheck);
        play(&mut cte, "Bb4+  Kf1 Nf6  Bg2 0-0  Ne5");

        invalid_turn(&mut cte, "Qxd1+", GameError::KingIsInCheckmate);
        play(&mut cte, "Qxd1#");

        // Game over again
        assert_eq!(Gamestate::Victory("Black".to_owned()), cte.gamestate());

        cte.display_on_screen(DisplayOption::BoardView(ViewMode::FancyTui));
    }

    #[test]
    /// [Event "Budapest m"]
    /// [Site "Budapest"]
    /// [Date "1895.12.03"]
    /// [Round "11"]
    /// [White "Maroczy, Geza"]
    /// [Black "Charousek, Rudolf Rezso"]
    /// [Result "1-0"]
    /// [WhiteElo ""]
    /// [BlackElo ""]
    /// [ECO "B06"]
    fn marcozy_vs_charousek() {
        let mut cte = ChessTurnEngine::new(Setup::Normal).unwrap();

        play(&mut cte, "d4 g6");
        play(&mut cte, "e4 d6");
        play(&mut cte, "f4 Bg7");
        play(&mut cte, "Nf3 Nd7");
        play(&mut cte, "Bd3 e5");
        play(&mut cte, "c3 Ngf6");
        play(&mut cte, "fxe5 dxe5");
        play(&mut cte, "dxe5 Nc5");
        play(&mut cte, "Bb5+ Nfd7");
        play(&mut cte, "Bg5 f6");
        play(&mut cte, "exf6 Bxf6");
        play(&mut cte, "Bxf6 Qxf6");
        play(&mut cte, "Qc2 Qb6");
        play(&mut cte, "Na3 a6");
        play(&mut cte, "Bc4 Nf6");
        play(&mut cte, "O-O-O Bd7");
        play(&mut cte, "Ne5 Be6");
        play(&mut cte, "b4 Ncxe4");
        play(&mut cte, "Rhe1 Rd8");
        play(&mut cte, "Rxd8+ Kxd8");
        play(&mut cte, "Rxe4 Nxe4");
        play(&mut cte, "Qxe4 Qg1+");
        play(&mut cte, "Kb2 Qf2+");
        play(&mut cte, "Nc2 Bxc4");
        play(&mut cte, "Qxc4 Kc8");
        play(&mut cte, "Qe6+ Kb8");
        play(&mut cte, "Nd7+ Ka8");
        play(&mut cte, "Qd5 Qf5");
        play(&mut cte, "Ne3 Qxd5");
        play(&mut cte, "Nxd5 Rd8");
        play(&mut cte, "N5f6 b6");
        play(&mut cte, "c4 Kb7");
        play(&mut cte, "Kc3 Kc8");
        play(&mut cte, "Kd4 Rxd7+");
        play(&mut cte, "Nxd7 Kxd7");
        play(&mut cte, "Kd5 a5");
        play(&mut cte, "bxa5 bxa5");
        play(&mut cte, "Kc5 c6");
        play(&mut cte, "Kb6 Kd6");
        play(&mut cte, "c5+");
        // Thoughts: "draw" option could be implemented into notation somehow
    }

    #[test]
    fn aron_nimzowitsch_vs_siegbert_tarrasch() {
        let mut cte = ChessTurnEngine::new(Setup::Normal).unwrap();

        play(&mut cte, "d4 d5");
        play(&mut cte, "Nf3 c5");
        play(&mut cte, "c4 e6");
        play(&mut cte, "e3 Nf6");
        play(&mut cte, "Bd3 Nc6");
        play(&mut cte, "O-O Bd6");
        play(&mut cte, "b3 O-O");
        play(&mut cte, "Bb2 b6");
        play(&mut cte, "Nbd2 Bb7");
        play(&mut cte, "Rc1 Qe7");
        play(&mut cte, "cxd5 exd5");
        play(&mut cte, "Nh4 g6");
        play(&mut cte, "Nhf3 Rad8");
        play(&mut cte, "dxc5 bxc5");
        play(&mut cte, "Bb5 Ne4");
        play(&mut cte, "Bxc6 Bxc6");
        play(&mut cte, "Qc2 Nxd2");
        play(&mut cte, "Nxd2 d4");
        play(&mut cte, "exd4 Bxh2+");
        play(&mut cte, "Kxh2 Qh4+");
        play(&mut cte, "Kg1 Bxg2");
        play(&mut cte, "f3 Rfe8");
        play(&mut cte, "Ne4 Qh1+");
        play(&mut cte, "Kf2 Bxf1");
        play(&mut cte, "d5 f5");
        play(&mut cte, "Qc3 Qg2+");
        play(&mut cte, "Ke3 Rxe4+");
        play(&mut cte, "fxe4 f4+");
        play(&mut cte, "Kxf4 Rf8+");
        play(&mut cte, "Ke5 Qh2+");
        play(&mut cte, "Ke6 Re8+");
        play(&mut cte, "Kd7 Bb5#");

        assert_eq!(Gamestate::Victory("Black".to_owned()), cte.gamestate());

        cte.display_on_screen(DisplayOption::BoardView(ViewMode::FancyTui));
        cte.display_on_screen(DisplayOption::TurnHistory);
        cte.display_on_screen(DisplayOption::CaptureHistory);
    }

    #[test]
    fn draw_after_only_kings_remain() {
        let mut cte = ChessTurnEngine::new(Setup::Normal).unwrap();

        // Below game was randomly generated for the first 40 turns :-)
        play(&mut cte, "g3 b5");
        play(&mut cte, "Nh3 d6");
        play(&mut cte, "Ng5 e6");
        play(&mut cte, "Nxh7 Rxh7");
        play(&mut cte, "a4 bxa4");
        play(&mut cte, "Rxa4 Rxh2");
        play(&mut cte, "Rxh2 Kd7");
        play(&mut cte, "Rxa7 Rxa7");
        play(&mut cte, "e3 Nf6");
        play(&mut cte, "Bb5+ Ke7");
        play(&mut cte, "Rh3 Bb7");
        play(&mut cte, "Rh1 Bxh1");
        play(&mut cte, "Bc6 Nxc6");
        play(&mut cte, "Qe2 Ra1");
        play(&mut cte, "d3 Rxb1");
        play(&mut cte, "f4 Rxc1+");
        play(&mut cte, "Qd1 Rxd1+");
        play(&mut cte, "Kxd1 Bf3+");
        play(&mut cte, "Kc1 Ne8");
        play(&mut cte, "Kb1 Nb8");
        play(&mut cte, "f5 exf5");
        play(&mut cte, "g4 Bxg4");
        play(&mut cte, "b3 f4");
        play(&mut cte, "exf4 Be2");
        play(&mut cte, "c4 Bxd3+");
        play(&mut cte, "Kc1 Bxc4");
        play(&mut cte, "bxc4 Qc8");
        play(&mut cte, "Kd1 Qg4+");
        play(&mut cte, "Ke1 Qh4+");
        play(&mut cte, "Ke2 Qxf4");
        play(&mut cte, "Kd3 Qd4+");
        play(&mut cte, "Kxd4 c5+");
        play(&mut cte, "Kc3 Kd7");
        play(&mut cte, "Kd3 Ke7");
        play(&mut cte, "Kc3 Nc6");
        play(&mut cte, "Kc2 Nd4+");
        play(&mut cte, "Kd3 d5");
        play(&mut cte, "cxd5 c4+");
        play(&mut cte, "Kxd4 Kd6");
        play(&mut cte, "Kxc4 Kc7");
        play(&mut cte, "d6+ Bxd6");
        play(&mut cte, "Kc3 Be5+");
        play(&mut cte, "Kd2 Bc3+");
        play(&mut cte, "Kxc3 Kb6");
        play(&mut cte, "Kd4 Nd6");
        play(&mut cte, "Kd5 f6");
        play(&mut cte, "Kxd6 g6");
        play(&mut cte, "Ke6 Ka6");
        play(&mut cte, "Kxf6 g5");
        play(&mut cte, "Kxg5");

        assert_eq!(
            Gamestate::DrawInsufficientMatingMaterial,
            cte.gamestate()
        );
    }

    #[test]
    fn draw_by_three_fold_repetition() {
        let mut cte = ChessTurnEngine::new(Setup::Normal).unwrap();

        play(&mut cte, "d4 d5");
        play(&mut cte, "Qd2 Qd7  Qd1 Qd8");
        play(&mut cte, "Qd2 Qd7  Qd1 Qd8");
        play(&mut cte, "Qd2");
        invalid_turn(
            &mut cte,
            "Qd7",
            GameError::GameOver(Gamestate::DrawThreeFoldRepetition),
        );
        assert_eq!(cte.gamestate(), Gamestate::DrawThreeFoldRepetition);
    }

    #[test]
    fn draw_by_fifty_move_rule() {
        let mut cte = ChessTurnEngine::new(Setup::Normal).unwrap();

        // Another randomly generated game
        play(&mut cte, "h4 Na6  h5 b5  d3 g6");
        play(&mut cte, "Rh3 Nf6  Nd2 Rg8  Rb1 Bg7");
        play(&mut cte, "b4 Nxb4  Rb3 Bb7  Nc4 Nbd5");
        play(&mut cte, "Rc3 c6  e4 Kf8  Bd2 Nxh5");
        play(&mut cte, "a3 Nxc3  g4 b4  Qf3 e5");
        play(&mut cte, "g5 Qe7  Bf4 Qc5  Ne3 Na2");
        play(&mut cte, "Rg3 Rd8  Qh1 Rc8  Bxe5 Qe7");
        play(&mut cte, "Qh4 Qf6  Kd1 Qxe5  d4 Ke7");
        play(&mut cte, "Qh2 c5  Qg2 Qf6  f4 Qd6");
        play(&mut cte, "Ke2 Qe5  Nd5+ Bxd5  Ke3 Bf6");
        play(&mut cte, "Qh3 Bc6  f5 Rgf8  d5 Ke8");
        play(&mut cte, "Bc4 Rc7  Qxh5 Qxd5  Nh3 Ba8");
        play(&mut cte, "gxf6 bxa3  Rg5 Qe6  Qe2 Rb7");
        play(&mut cte, "Bd5 h5  Rg1 gxf5  Kf2 Nc1");
        play(&mut cte, "Rg4 Qe7  Bc6 Rb5  Qd2 Nd3+");
        play(&mut cte, "Ke3 Rb8  Kxd3 Rb3+  Kc4 Qe6+");
        play(&mut cte, "Bd5 a6  Qf4 Qxf6  Rg6 Qe6");
        play(&mut cte, "Rg2 d6  Nf2 Rh8  Bxe6 Bd5+");
        play(&mut cte, "Kxd5 a5  Qg5 Rc3  Ng4 fxe6+");
        play(&mut cte, "Kxd6 fxe4  Nh6 a4  Qf5 Rg8");
        play(&mut cte, "Rh2 e5  Rh1 h4  Qd7+ Kf8");
        play(&mut cte, "Nf7 Rh3  Kxe5 Rf3  Nd8 Rg2");
        play(&mut cte, "c3 a2  Qc7 Rxc3  Qb8 Rd3");
        play(&mut cte, "Ke6 a1=Q  Nb7+ Rd8  Qg3 Rb8");
        play(&mut cte, "Rg1 Qe1  Rh1 Qf1  Rg1 hxg3");
        play(&mut cte, "Ke5 Ke8  Rh1 Kf8  Ke6 Re2");
        play(&mut cte, "Rh2 Qc1  Nd8 Rb5  Rh5 a3");
        play(&mut cte, "Rh4 Ke8  Kf5 Kf8  Rh6 Ra2");
        play(&mut cte, "Re6 Rab2  Re5 c4  Kg6 Kg8");
        play(&mut cte, "Re6 Rh5  Rb6 Qa1  Rb7 Qf1");
        play(&mut cte, "Rc7 Qb1  Rb7 Rh4  Kf5 Rf2+");
        play(&mut cte, "Ke5 Rh1  Ke6 Qxb7  Nf7 Qb5");
        play(&mut cte, "Ne5 Rh3  Nxc4 Rf6+  Kxf6 Qc5");
        play(&mut cte, "Nd2 Qg5+  Kxg5 Kh8  Nb3 Kh7");
        play(&mut cte, "Na1 Kg8  Kf6 e3  Nb3 Rh5");
        play(&mut cte, "Kg6 Rd5  Na1 Rh5  Kxh5 Kh8");
        play(&mut cte, "Kh6 Kg8  Nb3 Kh8  Kg6 g2");
        play(&mut cte, "Kg5 e2  Kh6 g1=N  Nd4 e1=Q");
        play(&mut cte, "Nf5 Qf1  Ng7 Qg2  Nh5 Qc6+");
        play(&mut cte, "Kg5 Nf3+  Kf4 Qa6  Ng3 Qb7");
        play(&mut cte, "Kf5 Qd5+  Kf4 Ng5  Ne2 Nh3+");
        play(&mut cte, "Kg3 Kh7  Kxh3 Qe6+  Kh2 Qb6");
        play(&mut cte, "Kh1 Qe3  Ng3 Qc1+  Kh2 Qg1+");
        play(&mut cte, "Kxg1 a2  Nh5 a1=Q+  Kh2 Qg7");
        play(&mut cte, "Nf4 Qb2+  Ng2 Kg8  Kh3 Qb5");
        play(&mut cte, "Kg3 Qa4  Nh4 Qxh4+  Kf3 Qd4");
        play(&mut cte, "Kg2 Qd7  Kg1 Qe8  Kf2 Qh5");
        play(&mut cte, "Kg3 Qf5  Kg2 Kh7  Kg3 Qb1");
        play(&mut cte, "Kg4 Kg6  Kh4 Qd3  Kg4 Qb1");
        play(&mut cte, "Kh3 Kg5  Kh2 Qh1+  Kg3 Qc6");
        play(&mut cte, "Kf2 Qb7  Ke3 Qf7  Ke4 Kg6");
        play(&mut cte, "Ke5 Qf5+  Kd6 Qf1  Kd5 Qb1");
        play(&mut cte, "Kc6 Qd3  Kb6 Qa3  Kb7 Kg5");
        play(&mut cte, "Kb8 Qg3+  Ka7 Kg4  Ka8 Kh4");
        play(&mut cte, "Kb7 Qe5  Ka6 Qd4  Kb7 Qd2");
        play(&mut cte, "Ka7 Qh6  Ka8 Qf6  Ka7 Kg4");
        play(&mut cte, "Kb7 Qf5  Ka7 Kf3  Ka6 Qf7");
        play(&mut cte, "Kb5 Qf4  Kb6 Qe4  Ka5 Kg4");
        play(&mut cte, "Kb6 Qd3  Kc5 Qc3+  Kd6 Qc2");
        play(&mut cte, "Ke7 Qe2+  Kd6 Qd2+  Ke6 Qe1+");
        play(&mut cte, "Kd6 Qe4  Kc7 Kg3  Kd7 Qh7+");
        play(&mut cte, "Kc6 Qh1+  Kd7 Qe1  Kd6 Qe6+");
        play(&mut cte, "Kc5");

        assert_eq!(cte.gamestate(), Gamestate::DrawFiftyMoveRule);
    }

    /// Play/undo/play so we test 'undo' functionality on every played turn
    fn play(cte: &mut ChessTurnEngine, turns: &str) {
        turns.split_whitespace().for_each(|turn| {
            // Play once
            eprintln!(" ----- playing turn: {}", turn);
            assert!(cte.play_turn(turn).is_ok());

            // Undo once
            assert!(cte.undo_turn().is_ok());

            // Make sure turn is available
            assert!(available_turns_contain_turn(
                cte.available_turns(),
                turn
            ));

            // Play it again
            assert!(cte.play_turn(turn).is_ok());
            cte.display_on_screen(DisplayOption::BoardView(ViewMode::FancyTui));
            eprintln!(" ######### played turn {}", turn);
        });
    }

    /// Try to play an invalid turn
    fn invalid_turn(cte: &mut ChessTurnEngine, turn: &str, err: GameError) {
        // Make sure turn is really not available
        assert!(!available_turns_contain_turn(
            cte.available_turns(),
            turn
        ));

        assert_eq!(cte.play_turn(turn), Err(err));
    }

    /// Match if given turn is contained in list of available turns
    fn available_turns_contain_turn(
        turns: &Vec<AvailableTurn>,
        turn: &str,
    ) -> bool {
        turns
            .iter()
            .map(|turn| turn.get_turn())
            .collect::<Vec<&str>>()
            .contains(
                &turn
                    .trim_matches(|c| c == '?' || c == '!') // remove comments
                    .to_owned()
                    .replace("O", "0")
                    .as_str(),
            )
    }

    /// Undo provided number of turns
    fn undo_turns(cte: &mut ChessTurnEngine, num_of_turns: usize) {
        (0..num_of_turns).for_each(|_| {
            assert!(cte.undo_turn().is_ok());
        });
    }
}
