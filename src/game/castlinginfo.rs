use super::side::Side;
use chess_notation_parser::{CastlingType, Square};

/// Path movement from source square to destination square
pub struct Path {
    /// Source
    pub src: Square,

    /// Destination
    pub dst: Square,
}

/// Return king's path
pub fn get_path_king(side: Side, castling_type: CastlingType) -> Path {
    match side {
        Side::White => match castling_type {
            CastlingType::Long => Path {
                src: Square::E1,
                dst: Square::C1,
            },
            CastlingType::Short => Path {
                src: Square::E1,
                dst: Square::G1,
            },
        },
        Side::Black => match castling_type {
            CastlingType::Long => Path {
                src: Square::E8,
                dst: Square::C8,
            },
            CastlingType::Short => Path {
                src: Square::E8,
                dst: Square::G8,
            },
        },
    }
}

/// Return rook's path
pub fn get_path_rook(side: Side, castling_type: CastlingType) -> Path {
    match side {
        Side::White => match castling_type {
            CastlingType::Long => Path {
                src: Square::A1,
                dst: Square::D1,
            },
            CastlingType::Short => Path {
                src: Square::H1,
                dst: Square::F1,
            },
        },
        Side::Black => match castling_type {
            CastlingType::Long => Path {
                src: Square::A8,
                dst: Square::D8,
            },
            CastlingType::Short => Path {
                src: Square::H8,
                dst: Square::F8,
            },
        },
    }
}

/// Return squares over which king has to cross in order to reach final castling
/// formation. Those squares must be check-free in order to make castling valid.
pub fn get_king_crossing_squares(
    side: Side,
    castling_type: CastlingType,
) -> &'static [Square] {
    match side {
        Side::White => match castling_type {
            CastlingType::Long => &[Square::D1, Square::C1],
            CastlingType::Short => &[Square::F1, Square::G1],
        },
        Side::Black => match castling_type {
            CastlingType::Long => &[Square::D8, Square::C8],
            CastlingType::Short => &[Square::F8, Square::G8],
        },
    }
}

/// Return squares all squares between the king and the rook.
/// Those squares must be empty in order to make castling valid.
pub fn get_required_empty_squares(
    side: Side,
    castling_type: CastlingType,
) -> &'static [Square] {
    match side {
        Side::White => match castling_type {
            CastlingType::Long => &[Square::D1, Square::C1, Square::B1],
            CastlingType::Short => &[Square::F1, Square::G1],
        },
        Side::Black => match castling_type {
            CastlingType::Long => &[Square::D8, Square::C8, Square::B8],
            CastlingType::Short => &[Square::F8, Square::G8],
        },
    }
}
