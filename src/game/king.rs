use super::board::Board;
use super::board_map::BoardMap;
use super::movement::{self, PieceMove};
use super::side::Side;
use chess_notation_parser::Square;

/// King can be in three possible states during the game
#[derive(PartialEq)]
pub enum KingState {
    /// King is safe
    Safe,

    /// King is under attack
    Check,

    /// King is possibly checkmated. It is in check and it cannot move safely.
    /// But we still ought to see if any other pieces can remove the check.
    SoftCheckmate,
}

/// Calculate king's state
pub fn get_state(board: &mut Board, side: Side) -> KingState {
    let square = board.get_king_pos(side);

    if is_safe(&board.map, square, side) {
        return KingState::Safe;
    }

    match any_safe_moves(&mut board.map, square, side) {
        true => KingState::Check,
        _ => KingState::SoftCheckmate,
    }
}

/// Check if king can make a safe move
fn any_safe_moves(map: &mut BoardMap, square: Square, side: Side) -> bool {
    let king_moves =
        movement::possible_squares_for_src(map, square, side, PieceMove::King);

    // Temporary pick up the king to make scanning easier
    let king = map.remove(&square);

    // Assume checkmate unless proven otherwise
    let mut any_safe_moves = false;

    for square in king_moves {
        if is_safe(map, square, side) {
            any_safe_moves = true;
            break;
        }
    }

    // Place the king back on its square
    map.insert(square, king.unwrap());

    any_safe_moves
}

/// Check if given location is safe for the king
pub fn is_safe(map: &BoardMap, pos: Square, side: Side) -> bool {
    let opponent = side.opponent();

    !(is_attacked_by(map, pos, opponent, PieceMove::Queen)
        || is_attacked_by(map, pos, opponent, PieceMove::Knight)
        || is_attacked_by(map, pos, opponent, PieceMove::Bishop)
        || is_attacked_by(map, pos, opponent, PieceMove::Rook)
        || is_attacked_by(map, pos, opponent, PieceMove::PawnCapture)
        || is_attacked_by(map, pos, opponent, PieceMove::King))
}

/// Check whether given piece is attacking the king
fn is_attacked_by(
    map: &BoardMap,
    pos: Square,
    side: Side,
    piece_move: PieceMove,
) -> bool {
    let squares =
        movement::possible_squares_for_dst(map, pos, side, piece_move);

    for square in squares.iter() {
        if let Some((p, _)) = map.get(square) {
            match p {
                piece if piece == piece_move.to_piece() => return true,
                _ => continue,
            }
        }
    }

    false
}
