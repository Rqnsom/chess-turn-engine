use super::board::Board;
use super::board_map::BoardMap;
use super::castlinginfo;
use super::enpassant::Enpassant;
use super::king::{self, KingState};
use super::movement::{self, PieceMove};
use super::play;
use super::state::State;
use super::{AvailableTurn, Side};
use chess_notation_parser::{
    turn_castling, turn_move, Castling, CastlingType, Flag, FlagCheck, Move,
    Piece, Square, Turn,
};

/// Square/Piece/Side - Basic info about chessboard pieces
struct Sps {
    square: Square,
    piece: Piece,
    side: Side,
}

/// Return list of all possible turns that are valid and can be played
pub fn get_available_turns(board: &mut Board) -> Vec<AvailableTurn> {
    let mut available_turns = Vec::<AvailableTurn>::with_capacity(128);
    let side = board.active_player;

    // Square/Piece/Side
    let player: Vec<Sps> = scan_for_pieces(&board.map, side);

    for sps in player.iter() {
        let moves = &mut get_turns(sps, board);
        available_turns.append(moves);
    }

    available_turns
}

/// Find info for every piece for a given player
fn scan_for_pieces(map: &BoardMap, side: Side) -> Vec<Sps> {
    map.into_iter()
        .filter(|(_, (_, s))| side == *s)
        .map(|(sq, (p, _))| Sps {
            square: sq,
            piece: p,
            side,
        })
        .collect::<Vec<Sps>>()
}

/// Preparation for `AvailableTurn` struct
struct TurnInfo {
    captured: Option<Piece>,
    turn: Turn,
}

/// Get all possible turns for a given piece
fn get_turns(sps: &Sps, board: &mut Board) -> Vec<AvailableTurn> {
    let unchecked_turns = get_unchecked_turns(sps, board);
    let mut turns = get_check_checkmate_flags(unchecked_turns, sps, board);

    set_correct_src(&mut turns, &board.map, sps);
    gen_available_turns(turns, sps)
}

/// Generate available turns.
/// Transform all `Turn' structs into `AvailableTurn` structs.
fn gen_available_turns(turns: Vec<TurnInfo>, sps: &Sps) -> Vec<AvailableTurn> {
    turns
        .iter()
        .map(|turn_info| {
            let dst = match turn_info.turn {
                Turn::Move(ref turn) => turn.dst,
                Turn::Castling(castling) => {
                    castlinginfo::get_path_king(sps.side, castling.r#type).dst
                }
            };

            AvailableTurn::new(
                sps.square.to_string(),
                dst.to_string(),
                sps.piece.to_string(),
                turn_info.captured.map(|piece| piece.to_string()),
                turn_info.turn.to_string(),
            )
        })
        .collect::<Vec<AvailableTurn>>()
}

/// Decide what must be set as a `src` in given `Turn`
fn set_correct_src(turns: &mut [TurnInfo], map: &BoardMap, sps: &Sps) {
    for turn_info in turns.iter_mut() {
        if let Turn::Move(ref mut move_turn) = turn_info.turn {
            move_turn.src = match sps.piece {
                Piece::Pawn => get_correct_pawn_src(move_turn, sps),
                _ => get_correct_non_pawn_src(move_turn, map, sps),
            };
        }
    }
}

/// Decide correct `src` for a pawn turn
fn get_correct_pawn_src(t_move: &Move, sps: &Sps) -> Option<Vec<Square>> {
    match t_move.check_flag(Flag::CAPTURE) {
        // Pawns require annotated source only in case of capture
        false => None,
        _ => Some(Square::get_file(sps.square.get_file_char()).unwrap()),
    }
}

/// Decide correct `src` for a non-pawn turn
fn get_correct_non_pawn_src(
    t_move: &Move,
    map: &BoardMap,
    sps: &Sps,
) -> Option<Vec<Square>> {
    let mut possible_src = movement::possible_squares_for_dst(
        map,
        t_move.dst,
        sps.side,
        PieceMove::from(sps.piece),
    );

    // We must always find at least one possible square which is sps.square
    if possible_src.len() == 1 {
        return None;
    }

    // Remove actual originating square
    possible_src.retain(|s| *s != sps.square);

    // TODO: Compress repetition below
    let src_file = sps.square.get_file_char();
    let mut use_file = true;
    for square in possible_src.iter() {
        if src_file == square.get_file_char() {
            use_file = false;
            break;
        }
    }

    if use_file {
        return Some(Square::get_file(src_file).unwrap());
    }

    let src_rank = sps.square.get_rank_char();
    let mut use_rank = true;
    for square in possible_src.iter() {
        if src_rank == square.get_rank_char() {
            use_rank = false;
            break;
        }
    }

    if use_rank {
        return Some(Square::get_rank(src_rank).unwrap());
    }

    Some(vec![sps.square])
}

/// Return turns with dst, promotion and capture flag set
fn get_unchecked_turns(sps: &Sps, board: &mut Board) -> Vec<Turn> {
    let get_castling = |turn: &Turn| -> CastlingType {
        match *turn {
            Turn::Castling(castling) => castling.r#type,
            _ => panic!("Castling expected"),
        }
    };

    let active_side = board.active_player;
    let mut unchecked_turns = match sps.piece {
        // Delicate pawns require special handling attention
        Piece::Pawn => return get_unchecked_pawn_turns(sps, board),

        // Include castling turns along with the king
        Piece::King => board
            .castling_rights
            .get()
            .into_iter()
            .filter(|(side, _)| *side == active_side)
            .map(|(_, castling)| turn_castling!(castling))
            .filter(|turn| {
                play::verify_castling(board, get_castling(turn)).is_ok()
            })
            .collect::<Vec<Turn>>(),
        _ => vec![],
    };

    unchecked_turns.append(&mut get_unchecked_non_pawn_turns(sps, board));
    unchecked_turns
}

/// Return non-pawn turns with dst, promotion and and capture flag setup
fn get_unchecked_non_pawn_turns(sps: &Sps, board: &Board) -> Vec<Turn> {
    movement::possible_squares_for_src(
        &board.map,
        sps.square,
        sps.side,
        PieceMove::from(sps.piece),
    )
    .iter()
    .map(|dst| {
        turn_move!(
            sps.piece,
            *dst,
            match board.map.get(dst).is_some() {
                true => Flag::CAPTURE,
                false => 0,
            }
        )
    })
    .collect::<Vec<Turn>>()
}

/// Return pawn turns with dst, promotion and and capture flag setup
fn get_unchecked_pawn_turns(sps: &Sps, board: &Board) -> Vec<Turn> {
    // Fetch normal turns first
    let mut turns = movement::possible_squares_for_src(
        &board.map,
        sps.square,
        sps.side,
        PieceMove::PawnNormal,
    )
    .into_iter()
    .filter(|square| board.map.get(square).is_none())
    .map(|dst| turn_move!(Piece::Pawn, dst, Flag::NONE))
    .collect::<Vec<Turn>>();

    // Fetch capture turns
    let mut capture_turns = movement::possible_squares_for_src(
        &board.map,
        sps.square,
        sps.side,
        PieceMove::PawnCapture,
    )
    .into_iter()
    .filter(|square| match board.map.get(square) {
        None => match board.enpassant {
            None => false,
            Some(enpassant) => *square == enpassant.capture_pos,
        },
        Some((p, s)) => s != board.active_player && p != Piece::King,
    })
    .map(|dst| turn_move!(Piece::Pawn, dst, Flag::CAPTURE))
    .collect::<Vec<Turn>>();

    turns.append(&mut capture_turns);
    turns = get_promotion_for_unchecked_pawn_turns(turns, board.active_player);
    turns
}

/// Generate promotion turns for pawns that reach it's final rank
fn get_promotion_for_unchecked_pawn_turns(
    mut turns: Vec<Turn>,
    side: Side,
) -> Vec<Turn> {
    let mut promotion_moves: Vec<Move> = vec![];
    let mut turns_iter = turns.iter_mut();

    while let Some(Turn::Move(turn)) = turns_iter.next() {
        if (turn.dst.get_rank_char() != '1' || side != Side::Black)
            && (turn.dst.get_rank_char() != '8' || side != Side::White)
        {
            continue;
        }

        // Use current `Move` to promote to queen
        turn.promotion = Some(Piece::Queen);

        let mut gen_promoted_turn = |piece| {
            let mut new_move: Move = turn.clone();
            new_move.promotion = Some(piece);
            promotion_moves.push(new_move);
        };

        // Generate rest of the promotion moves
        gen_promoted_turn(Piece::Rook);
        gen_promoted_turn(Piece::Bishop);
        gen_promoted_turn(Piece::Knight);
    }

    let mut promotion_turns = promotion_moves
        .into_iter()
        .map(Turn::Move)
        .collect::<Vec<Turn>>();

    turns.append(&mut promotion_turns);
    turns
}

/// Play the turn and check validity of the turn
fn simulate_turn(
    board: &mut Board,
    sps: &Sps,
    turn: &Turn,
) -> Result<State, ()> {
    let ret_ok = match turn {
        Turn::Move(r#move) if sps.piece == Piece::Pawn => {
            simulate_pawn_move(board, sps, r#move)
        }
        Turn::Move(r#move) => simulate_move(board, sps, r#move),
        Turn::Castling(castling) => simulate_castling(board, castling),
    }?;

    board.hash_state_push();
    board.active_player.switch_side();
    Ok(ret_ok)
}

/// Simulate castling turn
fn simulate_castling(board: &mut Board, turn: &Castling) -> Result<State, ()> {
    let state = State::new(board, turn.to_string());

    let side = board.active_player;
    let castling = (side, turn.r#type);

    // Something is quite wrong if castling is not available
    assert!(
        board.castling_rights.remove(&castling),
        "Castling right not found"
    );

    let rook_path = castlinginfo::get_path_rook(side, turn.r#type);
    board.move_piece(rook_path.dst, rook_path.src);

    let king_path = castlinginfo::get_path_king(side, turn.r#type);
    board.move_piece(king_path.dst, king_path.src);

    // King safety have been ensured in `get_unchecked_turns` function,
    // but let's double check
    assert!(
        king::is_safe(&board.map, board.get_king_pos(side), side),
        "Castling: King is not safe"
    );

    // Remove any possibility for castling for this player
    let castling_opposite = (side, turn.r#type.opposite());
    board.castling_rights.remove(&castling_opposite);

    board.enpassant = None;

    Ok(state)
}

/// Simulate pawn move
fn simulate_pawn_move(
    board: &mut Board,
    sps: &Sps,
    turn: &Move,
) -> Result<State, ()> {
    let mut state = State::new(board, turn.to_string());
    let side = board.active_player;

    // I used to like pawns before doing all this bs :-)
    // No need to check validity of the turn here, it has been done before
    // when unchecked turns were fetched
    state.captured = match board.move_piece(turn.dst, sps.square) {
        None => match board.enpassant {
            None => None,
            Some(enpassant) => match turn.dst == enpassant.capture_pos {
                true => Some((
                    enpassant.pawn_src,
                    board.map.remove(&enpassant.pawn_src).unwrap(),
                )),
                false => None,
            },
        },
        Some(captured) => Some((turn.dst, captured)),
    };

    // Double check capturing logic due to en-passant and whole pawn complexity
    assert!(!turn.check_flag(Flag::CAPTURE) ^ state.captured.is_some());

    state.moving_piece_src = Some(sps.square);

    // If our king is not safe, undo the simulated move
    if !king::is_safe(&board.map, board.get_king_pos(side), side) {
        // Undo will swap sides, so we don't want to change playing side here
        board.undo(state);
        return Err(());
    }

    if let Some(promotion) = turn.promotion {
        board.map.insert(turn.dst, (promotion, sps.side));
    }

    board.fifty_move_rule = 0;

    // Update new en-passant state
    board.enpassant = match turn.check_flag(Flag::CAPTURE) {
        true => None,
        _ => Enpassant::try_from(sps.square, turn.dst, side),
    };

    Ok(state)
}

/// Move piece from src to dst if our king is not in check
fn simulate_move(
    board: &mut Board,
    sps: &Sps,
    turn: &Move,
) -> Result<State, ()> {
    let mut state = State::new(board, turn.to_string());
    let side = board.active_player;

    let captured = board.move_piece(turn.dst, sps.square);
    state.captured = captured.map(|captured| (turn.dst, captured));

    state.moving_piece_src = Some(sps.square);

    // If our king is not safe, undo the simulated move
    if !king::is_safe(&board.map, board.get_king_pos(side), side) {
        // Undo will swap sides, so we don't want to change playing side here
        board.undo(state);
        return Err(());
    }

    // Post success actions
    board.fifty_move_rule = 0;
    board.enpassant = None;
    play::handle_castling_status(board, sps.square, turn, &captured);

    Ok(state)
}

/// Update check and checkmate flags and prepare `captured` piece
fn get_check_checkmate_flags(
    mut turns: Vec<Turn>,
    sps: &Sps,
    board: &mut Board,
) -> Vec<TurnInfo> {
    let mut to_be_removed = Vec::<usize>::new();
    let mut captured = Vec::<Option<Piece>>::with_capacity(turns.len());

    for (i, turn) in turns.iter_mut().enumerate() {
        // `simulate_turn` swaps `active_player` side
        let simulated_state = simulate_turn(board, sps, turn);
        if simulated_state.is_err() {
            captured.push(None);
            to_be_removed.push(i);
            continue;
        }

        match king::get_state(board, board.active_player) {
            KingState::Safe => (),
            KingState::Check => add_turn_flag(turn, Flag::CHECK),
            // Check that king is really in checkmate
            _ => add_turn_flag(
                turn,
                match confirm_checkmate(board) {
                    true => Flag::CHECKMATE,
                    _ => Flag::CHECK,
                },
            ),
        }

        let simulated_state = simulated_state.unwrap();
        captured
            .push(simulated_state.captured.map(|(_, (captured, _))| captured));

        board.active_player.switch_side();
        board.hash_state_pop();
        board.undo(simulated_state);
    }

    // Remove those turns which endanger our king
    while let Some(i) = to_be_removed.pop() {
        turns.remove(i);
        captured.remove(i);
    }

    turns
        .into_iter()
        .zip(captured.into_iter())
        .map(|(turn, captured)| TurnInfo { turn, captured })
        .collect::<Vec<TurnInfo>>()
}

/// Opponent's king is in check and has no safe moves available
/// Check if any opponent pieces can remove the check
fn confirm_checkmate(board: &mut Board) -> bool {
    // Let's act like this is N+1th turn on the board
    let opponent: Vec<Sps> = scan_for_pieces(&board.map, board.active_player);

    for sps in opponent {
        for turn in get_unchecked_turns(&sps, board) {
            match simulate_turn(board, &sps, &turn) {
                // If we can play any simulated turn, it means that our king is
                // not in check after our turn. It implies the turn has removed
                // king from the check, therefore king was never in checkmate
                Ok(state) => {
                    board.active_player.switch_side();
                    board.hash_state_pop();
                    board.undo(state);
                    return false;
                }
                _ => continue,
            }
        }
    }

    true
}

/// Append turn flag
fn add_turn_flag(turn: &mut Turn, flag: u8) {
    match turn {
        Turn::Castling(ref mut castling) => castling.flags |= flag,
        Turn::Move(ref mut r#move) => r#move.flags |= flag,
    }
}
