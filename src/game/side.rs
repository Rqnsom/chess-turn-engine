use std::fmt;

/// Player
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Side {
    /// Black player
    Black,

    /// White player
    White,
}

impl Side {
    /// Get opponent
    pub fn opponent(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }

    /// Change side/player
    pub fn switch_side(&mut self) {
        *self = self.opponent();
    }
}

impl TryFrom<&str> for Side {
    type Error = &'static str;

    fn try_from(side: &str) -> Result<Self, Self::Error> {
        Ok(match side {
            "w" | "W" => Self::White,
            "b" | "B" => Self::Black,
            _ => return Err("Unknown character"),
        })
    }
}

impl TryFrom<char> for Side {
    type Error = &'static str;

    fn try_from(side: char) -> Result<Self, Self::Error> {
        Ok(match side {
            'w' | 'W' => Self::White,
            'b' | 'B' => Self::Black,
            _ => return Err("Unknown character"),
        })
    }
}

impl fmt::Display for Side {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
                Self::White => "White",
                Self::Black => "Black",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printing_side() {
        assert_eq!(Side::Black.to_string(), "Black");
        assert_eq!(Side::White.to_string(), "White");
    }

    #[test]
    fn opponent() {
        assert_eq!(Side::Black.opponent(), Side::White);
        assert_eq!(Side::White.opponent(), Side::Black);
    }

    #[test]
    fn switch_side() {
        const WHITE: Side = Side::White;
        let mut white = WHITE;
        white.switch_side();
        assert_ne!(white, WHITE);
        white.switch_side();
        assert_eq!(white, WHITE);
    }

    #[test]
    fn try_from_sucess() {
        assert_eq!(Side::Black, Side::try_from('b').unwrap());
        assert_eq!(Side::Black, Side::try_from('B').unwrap());
        assert_eq!(Side::Black, Side::try_from("b").unwrap());
        assert_eq!(Side::Black, Side::try_from("B").unwrap());

        assert_eq!(Side::White, Side::try_from('w').unwrap());
        assert_eq!(Side::White, Side::try_from('W').unwrap());
        assert_eq!(Side::White, Side::try_from("w").unwrap());
        assert_eq!(Side::White, Side::try_from("W").unwrap());
    }

    #[test]
    fn try_from_failure() {
        assert_eq!(Err("Unknown character"), Side::try_from('x'));
    }
}
