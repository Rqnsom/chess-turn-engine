use super::castling_rights::CastlingRights;
use super::enpassant::Enpassant;
use super::{Board, Side};
use chess_notation_parser::{Piece, Square};

/// Turn theoretically can have this maximum size:
///  max 2 ascii chars for dst
///  max 2 ascii chars for src
///  max 2 ascii chars for promotion
///  max 2 ascii chars for comment
///  max 2 ascii chars for flags (capture combined with check/checkmate)
///  max 1 ascii char for terminating character
/// sum: 11
///
///  Maximum size for long castling is 8 (including terminating character)
const TURN_STR_MAX: usize = 12;

/// States are used for tracking history of turns.
/// Secondary purpose of data here is to make turn undoing possible.
///
/// Optimally, we would like to have this struct as small as possible since for
/// every turn made, new state will be saved onto turn history memory stack.
pub struct State {
    /// Track source to avoid its recalculation
    pub moving_piece_src: Option<Square>,

    /// En-passant possibility refers for the next move
    pub enpassant: Option<Enpassant>,

    /// All info about a captured piece.
    ///
    /// Having all this makes undoing en-passant easy
    pub captured: Option<(Square, (Piece, Side))>,

    /// Fifty turn rule counter
    pub fifty_move_rule: u8,

    /// Castling rights
    pub castling_rights: CastlingRights,

    /// From turn data, we can fetch demotion info
    turn: [u8; TURN_STR_MAX],
}

impl State {
    /// Take a snapshot of board state
    pub fn new(board: &Board, next_turn: String) -> Self {
        let mut turn: [u8; TURN_STR_MAX] = [0; 12];

        next_turn.as_bytes().iter().enumerate().for_each(|(i, b)| {
            turn[i] = *b;
        });

        Self {
            turn,
            moving_piece_src: None,
            enpassant: board.enpassant,
            fifty_move_rule: board.fifty_move_rule,
            castling_rights: board.castling_rights,
            captured: None,
        }
    }

    /// Convert turn from byte array back to &str
    pub fn get_turn(&self) -> &str {
        std::str::from_utf8(&self.turn)
            .unwrap()
            .trim_end_matches('\0')
    }
}
