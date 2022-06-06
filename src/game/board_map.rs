use super::side::Side;
use chess_notation_parser::{Piece, Square};

const ARR_LEN: u8 = 64;

/// Map that keeps track location of the pieces on the board
///
/// Original solution was done with the HashMap, so this implementation
/// uses similar API
#[derive(Clone, Copy, Hash)]
pub struct BoardMap {
    /// 64 board chess squares
    arr: [u8; ARR_LEN as usize],

    /// Number of squares containing pieces
    len: u8,

    /// Helper index for the `next` iterator function
    ///
    /// It goes from 0..64
    iterator_idx: u8,
}

const PIECE_PAWN: u8 = 1;
const PIECE_ROOK: u8 = 2;
const PIECE_KNIGHT: u8 = 3;
const PIECE_BISHOP: u8 = 4;
const PIECE_QUEEN: u8 = 5;
const PIECE_KING: u8 = 6;
const MASK_PIECES: u8 = 0x07;

const SIDE_BLACK: u8 = 1 << 6;
const SIDE_WHITE: u8 = 1 << 7;
const MASK_SIDES: u8 = (1 << 6) + (1 << 7);

/// Convert bitmask to tuple
fn u8_to_figure(square_data: u8) -> (Piece, Side) {
    let side = match square_data & MASK_SIDES {
        SIDE_WHITE => Side::White,
        _ => Side::Black,
    };

    let piece = match square_data & MASK_PIECES {
        PIECE_PAWN => Piece::Pawn,
        PIECE_ROOK => Piece::Rook,
        PIECE_KNIGHT => Piece::Knight,
        PIECE_BISHOP => Piece::Bishop,
        PIECE_QUEEN => Piece::Queen,
        _ => Piece::King,
    };

    (piece, side)
}

/// Convert tuple to bitmask
fn u8_from_figure(piece: Piece, side: Side) -> u8 {
    let side = match side {
        Side::White => SIDE_WHITE,
        Side::Black => SIDE_BLACK,
    };

    let piece = match piece {
        Piece::Pawn => PIECE_PAWN,
        Piece::Rook => PIECE_ROOK,
        Piece::Knight => PIECE_KNIGHT,
        Piece::Bishop => PIECE_BISHOP,
        Piece::Queen => PIECE_QUEEN,
        Piece::King => PIECE_KING,
    };

    side + piece
}

impl Iterator for BoardMap {
    type Item = (Square, (Piece, Side));

    fn next(&mut self) -> Option<Self::Item> {
        // We must return None at least once after all 64 elements were checked
        if self.iterator_idx >= ARR_LEN {
            self.iterator_idx = 0;
            return None;
        }

        let mut ret = self.arr[self.iterator_idx as usize];
        self.iterator_idx += 1;

        while ret == 0 {
            if self.iterator_idx >= ARR_LEN {
                self.iterator_idx = 0;
                return None;
            }

            ret = self.arr[self.iterator_idx as usize];
            self.iterator_idx += 1;
        }

        Some((Square::from(self.iterator_idx - 1), u8_to_figure(ret)))
    }
}

impl BoardMap {
    /// Creates an empty `BoardMap`.
    pub fn new() -> Self {
        Self {
            arr: [0u8; ARR_LEN as usize],
            len: 0,
            iterator_idx: 0,
        }
    }

    /// Returns a value corresponding to the key.
    pub fn get(&self, square: &Square) -> Option<(Piece, Side)> {
        match self.arr[*square as usize] {
            0 => None,
            data => Some(u8_to_figure(data)),
        }
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, None is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned.
    pub fn insert(
        &mut self,
        square: Square,
        (piece, side): (Piece, Side),
    ) -> Option<(Piece, Side)> {
        let old_data = self.arr[square as usize];

        let new_data = u8_from_figure(piece, side);
        self.arr[square as usize] = new_data;

        match old_data {
            0 => {
                self.len += 1;
                None
            }
            _ => Some(u8_to_figure(old_data)),
        }
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map
    pub fn remove(&mut self, square: &Square) -> Option<(Piece, Side)> {
        let old_data = match self.arr[*square as usize] {
            0 => return None,
            data => data,
        };

        self.arr[*square as usize] = 0;
        self.len -= 1;
        Some(u8_to_figure(old_data))
    }

    /// Returns the number of elements in the map
    pub fn len(&self) -> usize {
        self.len as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator() {
        let mut map = BoardMap::new();

        assert_eq!(map.remove(&Square::A8), None);
        map.insert(Square::A8, (Piece::Pawn, Side::White));
        map.insert(Square::A2, (Piece::Pawn, Side::Black));
        map.insert(Square::H1, (Piece::Pawn, Side::White));
        map.insert(Square::H7, (Piece::Pawn, Side::White));
        map.insert(Square::H8, (Piece::Pawn, Side::White));

        let mut iter = map.into_iter();
        while let Some(_) = iter.next() {
            continue;
        }
    }
}
