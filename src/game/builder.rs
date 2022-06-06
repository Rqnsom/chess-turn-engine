use super::board::Board;
use super::board_map::BoardMap;
use super::castling_rights::{CastlingRights, StartingRights};
use super::gamestate::Gamestate;
use super::side::Side;
use super::simulation;
use super::state::State;
use super::Game;
use chess_notation_parser::{Piece, Square};
use std::collections::BTreeMap;
use std::collections::HashMap;

/// Create a board using a `setup` string
///
/// String is format is documented in lib.rs file
pub fn create(setup: &'static str) -> Result<Game, &'static str> {
    let (map, king) = setup_map_and_kings(setup)?;

    let castling_rights = match setup != super::NORMAL_SETUP {
        // Remove castling rights for custom setups
        true => CastlingRights::new(StartingRights::None),
        _ => CastlingRights::new(StartingRights::All),
    };

    let tree: BTreeMap<u64, u8> = BTreeMap::new();
    let mut board = Board {
        map,
        enpassant: None,
        king,
        castling_rights,
        active_player: Side::White,
        gamestate: Gamestate::Ongoing,
        fifty_move_rule: 0,
        state_hashes: tree,
    };

    let available_turns = simulation::get_available_turns(&mut board);

    Ok(Game {
        history: Vec::<State>::with_capacity(128),
        board,
        available_turns,
    })
}

/// Setup the board using `setup` argument
fn setup_map_and_kings(
    setup: &'static str,
) -> Result<(BoardMap, HashMap<Side, Square>), &str> {
    let mut map = BoardMap::new();
    // TODO: Use something simpler instead of HashMap
    let mut king_cache = HashMap::<Side, Square>::with_capacity(2);

    for square_info in setup.split_whitespace() {
        let (square, piece, side) = parse_sps(square_info)?;

        if piece == Piece::King && king_cache.insert(side, square).is_some() {
            return Err("Player cannot have more than one king");
        }

        if map.insert(square, (piece, side)).is_some() {
            return Err("Square already occupied");
        }
    }

    match king_cache.len() {
        2 => Ok((map, king_cache)),
        _ => Err("Game needs to have two kings"),
    }
}

/// SPS - short for 'square/piece/side'
fn parse_sps(s: &str) -> Result<(Square, Piece, Side), &'static str> {
    let mut sps_iter = s.split(',');

    let square = Square::try_from(sps_iter.next().unwrap())?;
    let side = Side::try_from(sps_iter.next().unwrap())?;
    let piece = Piece::try_from(sps_iter.next().unwrap())?;

    match sps_iter.next() != None {
        true => Err("BoardBuilder: Invalid numbers of commas received"),
        _ => Ok((square, piece, side)),
    }
}
