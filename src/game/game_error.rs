use super::gamestate::Gamestate;
use std::error;
use std::fmt;

/// Error codes
#[derive(Debug, PartialEq)]
pub enum GameError {
    /// Origin square not found for moving piece
    MovingPieceNotFound,

    /// Unexpected king state for the turn
    KingIsSafe,

    /// Unexpected king state for the turn
    KingIsInCheck,

    /// Unexpected king state for the turn
    KingIsInCheckmate,

    /// King movement during castling must not be under check
    KingCannotCastleSafetly,

    /// Upon executing the turn, king must remain safe
    OurKingMustBeSafe,

    /// Turn missing info about the capture
    CaptureNotSet,

    /// Capture of an ally piece is not allowed
    CaptureAlly,

    /// Capture expected, but capture piece missing on the board on the
    /// destination square
    NoCapturePiece,

    /// Game over
    GameOver(Gamestate),

    /// King can castle if it is not under check
    CastlingUnderCheck,

    /// Castling not possible
    ///  - Maybe king/rook has moved already and doing so made castling
    ///  unavailable
    CastlingUnavailable,

    /// Squares between a rook and a king must be empty in order to perform
    /// castling turn
    CastlingSquaresNotEmpty,

    /// Pawn moves:
    ///  - diagonally only by capture action
    ///  - straight in case of no capture action
    InvalidPawnMovement,

    /// Turn notation is not correct
    ParsingTurnFailed,

    /// Undo unavailable
    UndoNotAvailable,
}

impl error::Error for GameError {}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_str())
    }
}

impl GameError {
    fn get_str(&self) -> String {
        match self {
            Self::MovingPieceNotFound => {
                "Source square not found for the moving piece".to_string()
            }
            Self::NoCapturePiece => {
                "Nothing to capture, dst square empty".to_string()
            }
            Self::KingIsInCheck => "Invalid turn: King is in check".to_string(),
            Self::KingIsInCheckmate => {
                "Invalid turn: King is in checkmate".to_string()
            }
            Self::OurKingMustBeSafe => {
                "Invalid turn: Our king is in check".to_string()
            }
            Self::CaptureNotSet => {
                "Invalid turn: Unexpected capture".to_string()
            }
            Self::CaptureAlly => {
                "Capturing ally pieces not allowed".to_string()
            }
            Self::CastlingUnavailable => "Castling not available".to_string(),
            Self::GameOver(gamestate) => format!("Game over: {}", gamestate),
            Self::CastlingUnderCheck => {
                "King under check cannot castle check".to_string()
            }
            Self::KingCannotCastleSafetly => {
                "King cannot safetly perform castling".to_string()
            }
            Self::CastlingSquaresNotEmpty => {
                "Squares between rook and king must be empty for castling"
                    .to_string()
            }
            Self::KingIsSafe => {
                "Invalid turn: King not supposed to be safe".to_string()
            }
            Self::InvalidPawnMovement => {
                "Pawns move diagonally only by capture".to_string()
            }
            Self::ParsingTurnFailed => "Parsing turn failed".to_string(),
            Self::UndoNotAvailable => "Undo not available".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print() {
        let errors = [
            GameError::MovingPieceNotFound,
            GameError::KingIsSafe,
            GameError::KingIsInCheck,
            GameError::KingIsInCheckmate,
            GameError::KingCannotCastleSafetly,
            GameError::OurKingMustBeSafe,
            GameError::CaptureNotSet,
            GameError::CaptureAlly,
            GameError::NoCapturePiece,
            GameError::GameOver(Gamestate::Ongoing),
            GameError::CastlingUnderCheck,
            GameError::CastlingUnavailable,
            GameError::CastlingSquaresNotEmpty,
            GameError::InvalidPawnMovement,
            GameError::ParsingTurnFailed,
            GameError::UndoNotAvailable,
        ];

        errors.iter().for_each(|err| {
            // Unit test coverage test
            err.to_string();
        });
    }
}
