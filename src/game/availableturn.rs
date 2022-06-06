use std::fmt;

/// Data which describes the turn that can be played.
///
/// In case of a castling turn, king's source and destination squares will be
/// provided.
///
/// Exact turn written in the algebraic chess notation format contains spoilers,
/// so it should be fetched via `get_turn` function.
pub struct AvailableTurn {
    /// Source square
    pub src: String,

    /// Destination square
    pub dst: String,

    /// Piece making the move
    pub piece: String,

    /// Captured piece
    pub captured: Option<String>,

    /// Chess notation format of the turn
    turn: String,
}

impl fmt::Display for AvailableTurn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AvailableTurn (src: {}, dst: {}, piece: {}, captured: {:?}, \
             turn: {}",
            self.src, self.dst, self.piece, self.captured, self.turn
        )
    }
}

impl AvailableTurn {
    /// Create `AvailableTurn`
    pub fn new(
        src: String,
        dst: String,
        piece: String,
        captured: Option<String>,
        turn: String,
    ) -> AvailableTurn {
        Self {
            src,
            dst,
            piece,
            captured,
            turn,
        }
    }

    /// Fetch turn written in chess notation format
    ///
    /// Turn info contains spoilers about the turn (checkmate or check),
    /// that's why the `turn` string is not publicly provided in the struct
    pub fn get_turn(&self) -> &str {
        self.turn.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print() {
        let src = String::from("a2");
        let dst = String::from("a3");
        let turn = String::from("a3");
        let piece = String::from("Pawn");
        let captured: Option<String> = None;

        assert_eq!(
            format!(
                "AvailableTurn (src: {}, dst: {}, piece: {}, captured: {:?}, \
                 turn: {}",
                src, dst, piece, captured, turn
            ),
            AvailableTurn::new(
                src.clone(),
                dst.clone(),
                piece,
                captured,
                turn.clone()
            )
            .to_string()
        );
    }
}
