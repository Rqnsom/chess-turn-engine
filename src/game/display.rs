use super::super::{DisplayOption, ViewMode};
use super::side::Side;
use super::state::State;
use super::Game;
use chess_notation_parser::{Piece, Square};

/// Display a game into a `String`
pub fn display_game(game: &Game, opt: DisplayOption) -> String {
    match opt {
        DisplayOption::BoardView(ViewMode::FancyTui) => {
            tui_fancy::display_board_fancy(game)
        }
        DisplayOption::BoardView(_) => tui_ascii::display_board_ascii(game),
        DisplayOption::TurnHistory => display_history(&game.history),
        DisplayOption::CaptureHistory => display_captured(&game.history),
    }
}

fn construct_square(rank: char, file: char) -> Square {
    let mut square = String::with_capacity(4);
    square.push(file);
    square.push(rank);
    Square::try_from(square.as_str()).expect("Unknown file or rank")
}

fn print_piece((piece, side): (Piece, Side)) -> char {
    match (side, piece) {
        (Side::Black, Piece::King) => '♚',
        (Side::Black, Piece::Queen) => '♛',
        (Side::Black, Piece::Rook) => '♜',
        (Side::Black, Piece::Knight) => '♞',
        (Side::Black, Piece::Bishop) => '♝',
        (Side::Black, Piece::Pawn) => '♟',
        (Side::White, Piece::King) => '♔',
        (Side::White, Piece::Queen) => '♕',
        (Side::White, Piece::Rook) => '♖',
        (Side::White, Piece::Knight) => '♘',
        (Side::White, Piece::Bishop) => '♗',
        (Side::White, Piece::Pawn) => '♙',
    }
}

/// Print turn history
fn display_history(history: &Vec<State>) -> String {
    if history.is_empty() {
        return String::new();
    }

    let mut s = String::from('\n');
    for (i, state) in history.iter().enumerate() {
        s.push_str(
            match i % 2 == 0 {
                true => format!("\t{:2}. {} ", (i / 2) + 1, state.get_turn()),
                false => format!("{}\n", state.get_turn()),
            }
            .as_str(),
        );
    }

    s.push('\n');
    s
}

/// Print captured pieces
fn display_captured(history: &Vec<State>) -> String {
    if history.is_empty() {
        return String::new();
    }

    let mut s = String::from('\n');
    for (i, state) in history.iter().enumerate() {
        if let Some((_, captured)) = state.captured {
            s.push_str(
                format!(
                    "{} ({}: {})\n",
                    print_piece(captured),
                    (i / 2) + 1,
                    state.get_turn()
                )
                .as_str(),
            )
        }
    }

    s.push('\n');
    s
}

mod tui_fancy {
    use super::*;

    const C_WHITE: &str = "\x1b[40m\x1b[37m";
    const C_BLACK: &str = "\x1b[47m\x1b[30m";
    const C_RESET: &str = "\x1b[40m\x1b[0m";
    const C_FG_RED: &str = "\x1b[1;37m";
    const C_GRID: &str = "\x1b[44m";

    /// Print board with a colorful format
    pub fn display_board_fancy(game: &Game) -> String {
        let mut s = String::new();

        const GRID_UNIT_LEN: usize = 3;
        const GRID_LEN: usize = 2 * GRID_UNIT_LEN + 8 * GRID_UNIT_LEN;

        // Refresh screen and reposition the cursor at the top left corner
        s.push_str("\x1B[2J\x1B[1;1H");

        // Top grid
        s.push_str(&format!("{}{}\n", C_GRID, " ".repeat(GRID_LEN)));

        for rank in ('1'..='8').rev() {
            // Print 'rank' letter in front of every row
            s.push_str(&format!("{} {}{} ", C_GRID, C_FG_RED, rank));

            for file in 'a'..='h' {
                s.push_str(&print_square(rank, file, game));
            }
            s.push_str(&format!("{}{}\n", C_GRID, " ".repeat(GRID_UNIT_LEN)));
        }

        // Print 'file' letter at bottom of every file/column
        s.push_str(&format!("{}{}", C_GRID, " ".repeat(GRID_UNIT_LEN)));
        for file in 'a'..='h' {
            s.push_str(&format!(" {} ", file));
        }
        s.push_str(&format!("{}{} \n", " ".repeat(GRID_UNIT_LEN), C_RESET));

        s
    }

    fn print_square(rank: char, file: char, game: &Game) -> String {
        let square = construct_square(rank, file);

        let color = match (rank as u8 + file as u8) % 2 {
            0 => C_BLACK,
            _ => C_WHITE,
        };

        let piece = match game.board.map.get(&square) {
            Some((piece, side)) => print_piece((piece, side)),
            None => ' ',
        };

        format!("{} {} ", color, piece)
    }
}

mod tui_ascii {
    use super::*;

    pub fn display_board_ascii(game: &Game) -> String {
        let mut s = String::with_capacity(256);

        for rank in ('1'..='8').rev() {
            // Print 'rank' letter in front of every row
            s.push_str(&format!("{} ", rank));

            for file in 'a'..='h' {
                s.push_str(&print_square(rank, file, game));
            }
            s.push('\n');
        }

        // Print 'file' letter at bottom of every file/column
        s.push_str("  ");
        for file in 'a'..='h' {
            s.push_str(&format!(" {} ", file));
        }

        s
    }

    fn print_piece(piece: Piece) -> char {
        match piece {
            Piece::King => 'K',
            Piece::Queen => 'Q',
            Piece::Rook => 'R',
            Piece::Bishop => 'B',
            Piece::Knight => 'N',
            Piece::Pawn => 'P',
        }
    }

    fn print_side(side: Side) -> char {
        match side {
            Side::Black => 'b',
            Side::White => 'w',
        }
    }

    fn print_square(rank: char, file: char, game: &Game) -> String {
        let mut s = String::with_capacity(10);
        let square = construct_square(rank, file);

        match game.board.map.get(&square) {
            Some((piece, side)) => {
                s.push_str(&format!(
                    "{}{} ",
                    print_side(side),
                    print_piece(piece)
                ));
            }
            None => {
                let empty_square = match (rank as u8 + file as u8) % 2 {
                    0 => " + ",
                    _ => " - ",
                };
                s.push_str(empty_square);
            }
        }
        s
    }
}
