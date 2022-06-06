use super::side::Side;
use chess_notation_parser::Square;

/// Info required for a possible en-passant turn
#[derive(Hash, Copy, Clone)]
pub struct Enpassant {
    /// Location of the pawn which can be captured
    pub pawn_src: Square,

    /// Location on which capturing enemy pawn can land
    pub capture_pos: Square,
}

impl Enpassant {
    /// See if current turn makes en-passant plausible, return `None` if not
    pub fn try_from(src: Square, dst: Square, side: Side) -> Option<Self> {
        // From source point of view
        let calc_y_dir = |steps: i8| match side {
            Side::Black => -steps,
            Side::White => steps,
        };

        // En-passant is possible only after pawn makes a 2-square move
        let y_dir = calc_y_dir(2);
        if src.get_relative_neighbor(0, y_dir).unwrap_or(src) != dst {
            return None;
        }

        let y_dir = calc_y_dir(1);
        Some(Enpassant {
            pawn_src: dst,
            // Unwrap will always succeed
            capture_pos: src.get_relative_neighbor(0, y_dir).unwrap(),
        })
    }
}
