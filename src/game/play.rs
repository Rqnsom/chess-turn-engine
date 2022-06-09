use super::castlinginfo;
use super::enpassant::Enpassant;
use super::game_error::GameError;
use super::gamestate::Gamestate;
use super::king::{self, KingState};
use super::movement::{self, PieceMove};
use super::side::Side;
use super::simulation;
use super::{Board, State};
use chess_notation_parser::{Castling, CastlingType, Move, Turn};
use chess_notation_parser::{Flag, FlagCheck, Piece, Square};

/// Verify and play the turn and update the board state
pub fn next_turn(board: &mut Board, turn: &str) -> Result<State, GameError> {
    if board.gamestate != Gamestate::Ongoing {
        return Err(GameError::GameOver(board.gamestate.clone()));
    }

    let mut turn = match chess_notation_parser::Turn::try_from(turn) {
        Err(_) => return Err(GameError::ParsingTurnFailed),
        Ok(turn) => turn,
    };

    let prev_board_state = match turn {
        Turn::Castling(turn) => play_castling(board, &turn),
        Turn::Move(ref mut turn) => play_move(board, turn),
    };

    if let Err(e) = prev_board_state {
        return Err(e);
    }

    board.fifty_move_rule += 1;
    // Fifty moves per player totals to hundred
    if board.fifty_move_rule >= 100 {
        board.gamestate = Gamestate::DrawFiftyMoveRule;
    }

    // Push function checks threefold repetition rule and returns that state
    // if all condtions are met. Otherwise, it returns the current gamestate.
    board.gamestate = board.hash_state_push();

    if turn.is_checkmate() {
        board.gamestate = Gamestate::Victory(board.active_player.to_string());
    }

    board.active_player.switch_side();
    Ok(prev_board_state.unwrap())
}

/// Play castling turn
///
/// Play turn, if everything is valid, and update castling possibilities
fn play_castling(
    board: &mut Board,
    turn: &Castling,
) -> Result<State, GameError> {
    let state = State::new(board, turn.to_string());

    let side = board.active_player;
    let castling = (side, turn.r#type);

    if !board.castling_rights.remove(&castling) {
        return Err(GameError::CastlingUnavailable);
    }

    if let Err(e) = verify_castling(board, turn.r#type) {
        board.castling_rights.insert(castling);
        return Err(e);
    }

    let rook_path = castlinginfo::get_path_rook(side, turn.r#type);
    board.move_piece(rook_path.dst, rook_path.src);

    let king_path = castlinginfo::get_path_king(side, turn.r#type);
    board.move_piece(king_path.dst, king_path.src);

    if let Err(e) = verify_check_checkmate(board, turn.flags) {
        board.undo(state);
        return Err(e);
    }

    // Clear en-passant state for non-pawn turns
    board.enpassant = None;

    // Remove any possibility for castling for this player
    let castling_opposite = (side, turn.r#type.opposite());
    board.castling_rights.remove(&castling_opposite);

    Ok(state)
}

/// Checker whether castling rules are followed
pub fn verify_castling(
    board: &mut Board,
    castling_type: CastlingType,
) -> Result<(), GameError> {
    let side = board.active_player;

    if !king::is_safe(&board.map, board.get_king_pos(side), side) {
        return Err(GameError::CastlingUnderCheck);
    }

    if castlinginfo::get_required_empty_squares(side, castling_type)
        .iter()
        .any(|square| board.map.get(square).is_some())
    {
        return Err(GameError::CastlingSquaresNotEmpty);
    }

    for square in castlinginfo::get_king_crossing_squares(side, castling_type) {
        if !king::is_safe(&board.map, *square, side) {
            return Err(GameError::KingCannotCastleSafetly);
        }
    }

    Ok(())
}

/// Play the Move
///
/// `turn` is given as mut because function will move out the turn.src from it
fn play_move(board: &mut Board, turn: &mut Move) -> Result<State, GameError> {
    match turn {
        // Dedicated handling for pawns due to en-passant and promotion
        Move {
            who: Piece::Pawn, ..
        } => play_pawn(board, turn),
        Move { who: _, .. } => play_piece(board, turn),
    }
}

/// Play the pawn turn
///
/// Handle pawn moving forward, pawn capture, en-passant, promotion
fn play_pawn(board: &mut Board, turn: &mut Move) -> Result<State, GameError> {
    let mut state = State::new(board, turn.to_string());
    let mut pawn_src: Option<Square> = None;

    let capture = turn.check_flag(Flag::CAPTURE);

    // Source must be empty, unless pawn is capturing
    if turn.src.is_none() ^ !capture {
        return Err(GameError::InvalidPawnMovement);
    }

    let possible_src = movement::possible_squares_for_dst(
        &board.map,
        turn.dst,
        board.active_player,
        match capture {
            true => PieceMove::PawnCapture,
            _ => PieceMove::PawnNormal,
        },
    );

    let src = movement::get_exact_src(possible_src, turn.src.take())?;

    let is_enpassant = verify_pawn_capture(board, capture, turn.dst)?;
    let mut captured = board.move_piece(turn.dst, src);

    if is_enpassant {
        assert_eq!(captured, None, "Enpassant: turn.dst does not capture");
        pawn_src = Some(board.enpassant.as_ref().unwrap().pawn_src);
        captured = board.map.remove(&pawn_src.unwrap());
    }

    if let Some(promotion) = turn.promotion {
        board.map.insert(turn.dst, (promotion, board.active_player));
    }

    state.moving_piece_src = Some(src);
    if let Err(e) = verify_check_checkmate(board, turn.flags) {
        board.undo(state);
        return Err(e);
    }

    // Update new en-passant state
    board.enpassant = match capture {
        true => None,
        _ => Enpassant::try_from(src, turn.dst, board.active_player),
    };

    handle_castling_status(board, src, turn, &captured);

    if let Some(captured) = captured {
        state.captured = Some((
            match is_enpassant {
                true => pawn_src.unwrap(),
                _ => turn.dst,
            },
            captured,
        ));
    }

    // Pawn movement resets this rule
    board.fifty_move_rule = 0;

    Ok(state)
}

/// Play the non-pawn turn
fn play_piece(board: &mut Board, turn: &mut Move) -> Result<State, GameError> {
    let mut state = State::new(board, turn.to_string());
    let capture = turn.check_flag(Flag::CAPTURE);

    let possible_src = movement::possible_squares_for_dst(
        &board.map,
        turn.dst,
        board.active_player,
        PieceMove::from(turn.who),
    );

    let src = movement::get_exact_src(possible_src, turn.src.take())?;

    verify_capture(board, capture, turn.dst)?;
    let captured = board.move_piece(turn.dst, src);

    state.moving_piece_src = Some(src);
    if let Some(captured) = captured {
        state.captured = Some((turn.dst, captured));

        // Any capture resets this counter
        board.fifty_move_rule = 0;
    }

    if let Err(e) = verify_check_checkmate(board, turn.flags) {
        board.undo(state);
        return Err(e);
    }

    // Clear en-passant state for non-pawn turns
    board.enpassant = None;

    handle_castling_status(board, src, turn, &captured);

    Ok(state)
}

/// Check that pawn capture is valid (normal capture and en-passant)
fn verify_pawn_capture(
    board: &Board,
    capture: bool,
    dst: Square,
) -> Result<bool, GameError> {
    let err = match verify_capture(board, capture, dst) {
        // `Ok` implies normal capture
        Ok(()) => return Ok(false),

        // When no en-passant, normal error is returned
        Err(e) if board.enpassant.is_none() => return Err(e),

        // En-passant implies capture
        Err(e) if !capture => return Err(e),

        // This arm means en-passant should be checked
        Err(e) => e,
    };

    // At this point, enpassant is definitely `Something`
    let enpassant = board.enpassant.as_ref().unwrap();

    // Is en-passantable piece actually being captured
    if dst != enpassant.capture_pos {
        return Err(err);
    }

    // Must always succeed at this point
    verify_capture(board, capture, enpassant.pawn_src)
        .expect("En-passant should have succeeded");
    Ok(true)
}

/// Make sure capture is in alignment according to what turn does and says
fn verify_capture(
    board: &Board,
    capture: bool,
    dst: Square,
) -> Result<(), GameError> {
    match board.map.get(&dst) {
        None => match capture {
            false => Ok(()),
            _ => Err(GameError::NoCapturePiece),
        },
        Some((piece, side)) => match capture {
            false => Err(GameError::CaptureNotSet),
            _ => {
                assert_ne!(piece, Piece::King, "King cannot be captured");

                if side == board.active_player {
                    return Err(GameError::CaptureAlly);
                }
                Ok(())
            }
        },
    }
}

/// This function will check if opponent has any available turns.
///
/// Before using the function, `enpassant` and `castling_rights` should be
/// correctly set for the current 'turn'.
fn can_opponent_do_any_turns(board: &mut Board) -> bool {
    board.active_player.switch_side();
    let turns = simulation::get_available_turns(board);
    board.active_player.switch_side();

    !turns.is_empty()
}

/// Check status king status after the turn is played and check whether it is
/// in accordance to what turn states
fn verify_check_checkmate(
    board: &mut Board,
    flags: u8,
) -> Result<(), GameError> {
    let side = board.active_player;

    // Our king shouldn't be in check or checkmate
    if !king::is_safe(&board.map, board.get_king_pos(side), side) {
        return Err(GameError::OurKingMustBeSafe);
    }

    match king::get_state(board, side.opponent()) {
        KingState::Safe if flags & (Flag::CHECKMATE | Flag::CHECK) != 0 => {
            Err(GameError::KingIsSafe)
        }
        KingState::Check if flags & Flag::CHECK == 0 => {
            Err(GameError::KingIsInCheck)
        }
        // SoftCheckmate combined with the fact that no other pieces can move,
        // implies that no other pieces can stop the check,
        // therfore check is actually a checkmate
        KingState::SoftCheckmate
            if !can_opponent_do_any_turns(board)
                && flags & Flag::CHECKMATE == 0 =>
        {
            Err(GameError::KingIsInCheckmate)
        }
        _ => Ok(()),
    }
}

/// Track whether castling is still possible
/// Update castling rights based on
///  - king/rook movement
///  - captured rook
pub fn handle_castling_status(
    board: &mut Board,
    src: Square,
    turn: &Move,
    captured: &Option<(Piece, Side)>,
) {
    if board.castling_rights.is_empty() {
        return;
    }

    let side = board.active_player;
    match turn.who {
        // As soon as the king moves, castling right is forever gone
        Piece::King => {
            board.castling_rights.remove(&(side, CastlingType::Short));
            board.castling_rights.remove(&(side, CastlingType::Long));
        }
        Piece::Rook => match src {
            Square::A1 | Square::A8 => {
                board.castling_rights.remove(&(side, CastlingType::Long));
            }
            Square::H1 | Square::H8 => {
                board.castling_rights.remove(&(side, CastlingType::Short));
            }
            _ => (),
        },
        _ => (),
    }

    // Remove castling rights if 'idle' rook is captured
    if let Some((Piece::Rook, s)) = captured {
        match turn.dst {
            Square::A1 | Square::A8 => {
                board.castling_rights.remove(&(*s, CastlingType::Long));
            }
            Square::H1 | Square::H8 => {
                board.castling_rights.remove(&(*s, CastlingType::Short));
            }
            _ => (),
        }
    }
}
