use std::fmt;

/// Board game state
#[derive(Debug, Clone, PartialEq)]
pub enum Gamestate {
    /// Game still ongoing
    Ongoing,

    /// Type of a draw where player is not in check, but has no legal turns to
    /// make
    Stalemate,

    /// No player can win the game
    DrawInsufficientMatingMaterial,

    /// Draw by fifty move rule
    DrawFiftyMoveRule,

    /// Draw by three fold repetition rule
    DrawThreeFoldRepetition,

    /// Victory and name of the winner
    Victory(String),
}

impl fmt::Display for Gamestate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Victory(side) => format!("{} won", side),
                Self::Ongoing => "Ongoing".to_owned(),
                Self::Stalemate => "Stalemate".to_owned(),
                Self::DrawFiftyMoveRule => "Draw by fifty move rule".to_owned(),
                Self::DrawThreeFoldRepetition =>
                    "Draw by three fold repetition rule".to_owned(),
                Self::DrawInsufficientMatingMaterial =>
                    "Draw by insufficient mating material".to_owned(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print() {
        assert_eq!(Gamestate::Ongoing.to_string().as_str(), "Ongoing");
        assert_eq!(Gamestate::Stalemate.to_string().as_str(), "Stalemate");
        assert_eq!(
            Gamestate::DrawThreeFoldRepetition.to_string(),
            "Draw by three fold repetition rule"
        );
        assert_eq!(
            Gamestate::DrawFiftyMoveRule.to_string(),
            "Draw by fifty move rule"
        );
        assert_eq!(
            Gamestate::DrawInsufficientMatingMaterial.to_string(),
            "Draw by insufficient mating material"
        );
        assert_eq!(
            Gamestate::Victory("White".to_string()).to_string().as_str(),
            "White won"
        );
        assert_eq!(
            Gamestate::Victory("Black".to_string()).to_string().as_str(),
            "Black won"
        );
    }
}
