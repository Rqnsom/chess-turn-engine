use super::side::Side;
use chess_notation_parser::CastlingType;

/// Initialization info
///
/// Some chess games might omit castling rights
#[derive(PartialEq)]
pub enum StartingRights {
    All,
    None,
}

/// Struct which keeps track of which castling is possible
#[derive(Hash, Copy, Clone)]
pub struct CastlingRights {
    /// All info can fit within 8 bits
    bitmask: u8,
}

/// Castling masks
const CASTLING_WHITE_LONG: u8 = 1 << 0;
const CASTLING_WHITE_SHORT: u8 = 1 << 1;
const CASTLING_BLACK_LONG: u8 = 1 << 2;
const CASTLING_BLACK_SHORT: u8 = 1 << 3;

impl CastlingRights {
    /// Create new CastlingRights with given rights
    pub fn new(rights: StartingRights) -> CastlingRights {
        let mut ret = Self { bitmask: 0 };

        if rights == StartingRights::None {
            return ret;
        }

        ret.insert((Side::White, CastlingType::Long));
        ret.insert((Side::White, CastlingType::Short));
        ret.insert((Side::Black, CastlingType::Long));
        ret.insert((Side::Black, CastlingType::Short));
        ret
    }

    /// Get all castling rights packed in a vector
    pub fn get(&self) -> Vec<(Side, CastlingType)> {
        let mut ret = Vec::<(Side, CastlingType)>::with_capacity(4);

        if self.bitmask & CASTLING_WHITE_LONG != 0 {
            ret.push((Side::White, CastlingType::Long));
        }
        if self.bitmask & CASTLING_WHITE_SHORT != 0 {
            ret.push((Side::White, CastlingType::Short));
        }
        if self.bitmask & CASTLING_BLACK_LONG != 0 {
            ret.push((Side::Black, CastlingType::Long));
        }
        if self.bitmask & CASTLING_BLACK_SHORT != 0 {
            ret.push((Side::Black, CastlingType::Short));
        }

        ret
    }

    /// Insert castling right
    ///
    /// If the set did not have this value present, true is returned.
    ///
    /// If the set did have this value present, false is returned.
    pub fn insert(&mut self, (side, castling): (Side, CastlingType)) -> bool {
        let mask = Self::get_mask((side, castling));

        if mask & self.bitmask != 0 {
            return true;
        }

        self.bitmask |= mask;
        false
    }

    /// Remove castling right
    pub fn remove(&mut self, (side, castling): &(Side, CastlingType)) -> bool {
        let mask = Self::get_mask((*side, *castling));

        if mask & self.bitmask == 0 {
            return false;
        }

        self.bitmask &= !mask;
        true
    }

    /// Returns true if no castling rights available
    pub fn is_empty(&self) -> bool {
        self.bitmask == 0
    }

    /// Convert tuple into a bitmask
    fn get_mask(side_castling: (Side, CastlingType)) -> u8 {
        match side_castling {
            (Side::White, CastlingType::Long) => CASTLING_WHITE_LONG,
            (Side::White, CastlingType::Short) => CASTLING_WHITE_SHORT,
            (Side::Black, CastlingType::Long) => CASTLING_BLACK_LONG,
            (Side::Black, CastlingType::Short) => CASTLING_BLACK_SHORT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extra_insert_does_nothing() {
        let mut map = CastlingRights::new(StartingRights::None);

        let castling = (Side::White, CastlingType::Long);
        assert_eq!(map.insert(castling), false);
        assert_eq!(map.insert(castling), true);
        assert_eq!(map.remove(&castling), true);
        assert_eq!(map.remove(&castling), false);
    }
}
