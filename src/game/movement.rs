use super::board_map::BoardMap;
use super::game_error::GameError;
use super::side::Side;
use chess_notation_parser::{Piece, Square};

const DIR_N: Coordinate = Coordinate { x: 0, y: 1 };
const DIR_S: Coordinate = Coordinate { x: 0, y: -1 };
const DIR_E: Coordinate = Coordinate { x: 1, y: 0 };
const DIR_W: Coordinate = Coordinate { x: -1, y: 0 };

const DIR_NE: Coordinate = Coordinate { x: 1, y: 1 };
const DIR_NW: Coordinate = Coordinate { x: -1, y: 1 };
const DIR_SE: Coordinate = Coordinate { x: -1, y: -1 };
const DIR_SW: Coordinate = Coordinate { x: 1, y: -1 };

/// Pawn direction
enum PawnDir {
    Forward,
    Backward,
}

/// Piece movement
///
/// Purpose of this struct is to have clear pawn moving patterns depending on
/// their `capture` condition
#[derive(Clone, Copy)]
pub enum PieceMove {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,

    /// Pawn moving diagonally
    PawnCapture,

    /// Pawn moving forward
    PawnNormal,
}

impl PieceMove {
    /// Convert to piece
    pub fn to_piece(self) -> Piece {
        match self {
            PieceMove::Queen => Piece::Queen,
            PieceMove::King => Piece::King,
            PieceMove::Bishop => Piece::Bishop,
            PieceMove::Knight => Piece::Knight,
            PieceMove::Rook => Piece::Rook,
            _ => Piece::Pawn,
        }
    }
}

impl From<Piece> for PieceMove {
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::Queen => PieceMove::Queen,
            Piece::King => PieceMove::King,
            Piece::Bishop => PieceMove::Bishop,
            Piece::Knight => PieceMove::Knight,
            Piece::Rook => PieceMove::Rook,
            _ => panic!("Not implemented for pawns"),
        }
    }
}

/// Chessboard coordinates
#[derive(Clone, Copy)]
struct Coordinate {
    /// 'file' axis
    x: i8,

    /// 'rank' axis
    y: i8,
}

/// Description of movement pattern
///
/// Iterator which can be used to get all possible squares for a certain pattern
/// using the current position, direction and 'squares left' info.
struct MovePattern {
    /// Current position
    dst: Square,

    /// Steps left for the direction
    moves_left: u8,

    /// Direction
    dir: Coordinate,
}

impl Iterator for MovePattern {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.moves_left == 0 {
            return None;
        }

        let dst = match self.dst.get_relative_neighbor(self.dir.x, self.dir.y) {
            Err(_) => {
                self.moves_left = 0;
                return None;
            }
            Ok(square) => square,
        };

        self.moves_left -= 1;
        self.dst = dst;
        Some(dst)
    }
}

/// Get pattern for a given piece
///
/// Argument `side` is relevant only for pawns
fn get_move_pattern(
    square: Square,
    side: Side,
    piece_move: PieceMove,
    dir: PawnDir,
) -> Vec<MovePattern> {
    match piece_move {
        PieceMove::PawnNormal => get_moves_pawn_normal(square, side, dir),
        PieceMove::PawnCapture => get_moves_pawn_capture(square, side, dir),
        PieceMove::Queen => get_moves_queen(square),
        PieceMove::Bishop => get_moves_bishop(square),
        PieceMove::Rook => get_moves_rook(square),
        PieceMove::Knight => get_moves_knight(square),
        PieceMove::King => get_moves_king(square),
    }
}

/// Looking at the destination square, find all possible squares that
/// contain the moving piece which can actually move to the given destination
///
/// # Arguments
///
/// * `map` - board map
/// * `dst` - Destination square for the moving piece
/// * `side` - Color of the moving piece
/// * `piece_move` - Moving piece which is moving encapsulated in `PieceMove`
pub fn possible_squares_for_dst(
    map: &BoardMap,
    dst: Square,
    side: Side,
    piece_move: PieceMove,
) -> Vec<Square> {
    let dep_moves = get_move_pattern(dst, side, piece_move, PawnDir::Backward);
    let mut src_squares: Vec<Square> = vec![];

    // Filter out all possible source squares based on what we have on those
    // squares. We should find at least one piece of who we are looking for on
    // those squares.
    for mut dep_move in dep_moves {
        for square in dep_move.by_ref() {
            if let Some((p, s)) = map.get(&square) {
                if s == side && p == piece_move.to_piece() {
                    src_squares.push(square);
                }

                // We encontered anothere piece and we cannot jump over it.
                // Therefeore skip the direction, since it's blocking our path
                break;
            }
        }
    }

    src_squares
}

/// Get list of squares to which piece can move
///
/// # Arguments
///
/// * `map` - board map
/// * `src` - Source square for the moving piece
/// * `side` - Color of the moving piece
/// * `piece_move` - Moving piece which is moving encapsulated in `PieceMove`
pub fn possible_squares_for_src(
    map: &BoardMap,
    src: Square,
    side: Side,
    piece_move: PieceMove,
) -> Vec<Square> {
    let dep_moves = get_move_pattern(src, side, piece_move, PawnDir::Forward);
    let mut dst_squares: Vec<Square> = vec![];

    // Filter out all possible destination squares based on what we have on
    // those squares.
    for mut dep_move in dep_moves {
        for square in dep_move.by_ref() {
            if let Some((p, s)) = map.get(&square) {
                if s != side && p != Piece::King {
                    dst_squares.push(square);
                }

                // We encontered anothere piece and we cannot jump over it.
                // Therefeore skip the direction, since it's blocking our path
                break;
            }

            // We can move on empty squares
            dst_squares.push(square)
        }
    }

    dst_squares
}

/// Find exact source square by comparing possible squares with info about
/// the source squares that was received from the annotated turn
///
/// # Arguments
///
/// * `possible_src` - list of all possible source squares based on the current
///     board state.
/// * `turn_src` - Info about originating source square provided via turn
///     notation. e.g. dxe5 -> 'd' here represents the whole 'd' rank
pub fn get_exact_src(
    mut possible_src: Vec<Square>,
    turn_src: Option<Vec<Square>>,
) -> Result<Square, GameError> {
    let err = Err(GameError::MovingPieceNotFound);
    let mut exact_src_square = None;

    let turn_src: Vec<Square> = match turn_src {
        None if possible_src.len() != 1 => return err,
        None => return Ok(possible_src.pop().unwrap()),
        Some(src_squares) => src_squares,
    };

    for square in possible_src.into_iter() {
        if turn_src.contains(&square) {
            if exact_src_square.is_some() {
                return err;
            }
            exact_src_square = Some(square);
        }
    }

    match exact_src_square.is_none() {
        true => err,
        _ => Ok(exact_src_square.unwrap()),
    }
}

#[rustfmt::skip]
fn get_moves_pawn_normal(
    dst: Square,
    side: Side,
    dir: PawnDir
) -> Vec<MovePattern> {
    let mut moves_left = 1;

    match side {
        Side::White => {
            let (dir, rank) = match dir {
                PawnDir::Backward => (DIR_S, '4'),
                _ => (DIR_N, '2')
            };

            if Square::get_rank(rank).unwrap().contains(&dst) {
                moves_left = 2;
            };

            vec![MovePattern { dst, moves_left, dir }]
        },
        Side::Black => {
            let (dir, rank) = match dir {
                PawnDir::Backward => (DIR_N, '5'),
                _ => (DIR_S, '7')
            };

            if Square::get_rank(rank).unwrap().contains(&dst) {
                moves_left = 2;
            };

            vec![MovePattern { dst, moves_left, dir }]
        }
    }
}

#[rustfmt::skip]
fn get_moves_pawn_capture(
    dst: Square,
    side: Side,
    dir: PawnDir
) -> Vec<MovePattern> {
    match side {
        Side::White => {
            match dir {
                PawnDir::Backward => vec![
                    MovePattern { dst, moves_left: 1, dir: DIR_SE, },
                    MovePattern { dst, moves_left: 1, dir: DIR_SW, },
                ],
                _ => vec![
                    MovePattern { dst, moves_left: 1, dir: DIR_NE, },
                    MovePattern { dst, moves_left: 1, dir: DIR_NW, },
                ]
            }
        },
        Side::Black => {
            match dir {
                PawnDir::Backward => vec![
                    MovePattern { dst, moves_left: 1, dir: DIR_NE, },
                    MovePattern { dst, moves_left: 1, dir: DIR_NW, },
                ],
                _ => vec![
                    MovePattern { dst, moves_left: 1, dir: DIR_SE, },
                    MovePattern { dst, moves_left: 1, dir: DIR_SW, },
                ]
            }
        }
    }
}

#[rustfmt::skip]
fn get_moves_queen(dst: Square) -> Vec<MovePattern> {
    #[allow(non_upper_case_globals)]
    const moves_left: u8 = 8;

    vec![
        MovePattern { dst, moves_left, dir: DIR_N, },
        MovePattern { dst, moves_left, dir: DIR_NE, },
        MovePattern { dst, moves_left, dir: DIR_E, },
        MovePattern { dst, moves_left, dir: DIR_SE, },
        MovePattern { dst, moves_left, dir: DIR_S, },
        MovePattern { dst, moves_left, dir: DIR_SW, },
        MovePattern { dst, moves_left, dir: DIR_W, },
        MovePattern { dst, moves_left, dir: DIR_NW, },
    ]
}

#[rustfmt::skip]
fn get_moves_bishop(dst: Square) -> Vec<MovePattern> {
    #[allow(non_upper_case_globals)]
    const moves_left: u8 = 8;

    vec![
        MovePattern { dst, moves_left, dir: DIR_NE, },
        MovePattern { dst, moves_left, dir: DIR_SE, },
        MovePattern { dst, moves_left, dir: DIR_SW, },
        MovePattern { dst, moves_left, dir: DIR_NW, },
    ]
}

#[rustfmt::skip]
fn get_moves_rook(dst: Square) -> Vec<MovePattern> {
    #[allow(non_upper_case_globals)]
    const moves_left: u8 = 8;

    vec![
        MovePattern { dst, moves_left, dir: DIR_N, },
        MovePattern { dst, moves_left, dir: DIR_E, },
        MovePattern { dst, moves_left, dir: DIR_S, },
        MovePattern { dst, moves_left, dir: DIR_W, },
    ]
}

#[rustfmt::skip]
fn get_moves_king(dst: Square) -> Vec<MovePattern> {
    #[allow(non_upper_case_globals)]
    const moves_left: u8 = 1;

    vec![
        MovePattern { dst, moves_left, dir: DIR_N, },
        MovePattern { dst, moves_left, dir: DIR_NE, },
        MovePattern { dst, moves_left, dir: DIR_E, },
        MovePattern { dst, moves_left, dir: DIR_SE, },
        MovePattern { dst, moves_left, dir: DIR_S, },
        MovePattern { dst, moves_left, dir: DIR_SW, },
        MovePattern { dst, moves_left, dir: DIR_W, },
        MovePattern { dst, moves_left, dir: DIR_NW, },
   ]
}

#[rustfmt::skip]
fn get_moves_knight(dst: Square) -> Vec<MovePattern> {
    #[allow(non_upper_case_globals)]
    const moves_left: u8 = 1;

    vec![
        MovePattern { dst, moves_left, dir: Coordinate { x: 1, y: 2 } },
        MovePattern { dst, moves_left, dir: Coordinate { x: 1, y: -2 } },
        MovePattern { dst, moves_left, dir: Coordinate { x: -1, y: 2 } },
        MovePattern { dst, moves_left, dir: Coordinate { x: -1, y: -2 } },
        MovePattern { dst, moves_left, dir: Coordinate { x: 2, y: 1 } },
        MovePattern { dst, moves_left, dir: Coordinate { x: 2, y: -1 } },
        MovePattern { dst, moves_left, dir: Coordinate { x: -2, y: 1 } },
        MovePattern { dst, moves_left, dir: Coordinate { x: -2, y: -1 } },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn piece_move_from_piece_to_piece() {
        assert_eq!(PieceMove::from(Piece::Queen).to_piece(), Piece::Queen);
        assert_eq!(PieceMove::from(Piece::Knight).to_piece(), Piece::Knight);
        assert_eq!(PieceMove::from(Piece::King).to_piece(), Piece::King);
        assert_eq!(PieceMove::from(Piece::Bishop).to_piece(), Piece::Bishop);
        assert_eq!(PieceMove::from(Piece::Rook).to_piece(), Piece::Rook);

        let pawn = PieceMove::PawnCapture;
        assert_eq!(pawn.to_piece(), Piece::Pawn);

        // Let's panic here
        let _pawn_move = PieceMove::from(Piece::Pawn);
    }
}
