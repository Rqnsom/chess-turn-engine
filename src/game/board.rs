use super::board_map::BoardMap;
use super::castling_rights::CastlingRights;
use super::castlinginfo;
use super::enpassant::Enpassant;
use super::game_error::GameError;
use super::gamestate::Gamestate;
use super::play;
use super::side::Side;
use super::state::State;
use chess_notation_parser::{Castling, Move, Piece, Square, Turn};
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Board tracks state of the game.
pub struct Board {
    /// 64 squares containing all chess pieces
    pub map: BoardMap,

    /// Current player
    pub active_player: Side,

    /// Cache for location of the kings
    pub king: HashMap<Side, Square>,

    /// En-passant possibility
    ///
    /// Is updated every turn.
    /// If pawn moves two squares, then `enpassant` state is stored.
    /// Set enpassant allows enemy pawn to perform an en-passant action.
    /// If en-passant capture is taken while available, then next turn will
    /// reset it to `None`.
    pub enpassant: Option<Enpassant>,

    /// Track info whether certain castling is possible
    ///
    /// If `CastlingType` can be found in the struct, that castling is possible
    /// for that `Side`.
    pub castling_rights: CastlingRights,

    /// Game status info
    pub gamestate: Gamestate,

    /// Counter for the *fifty move rule*. If counter reaches 50, game is a draw
    ///
    /// Counter is incremented by default every turn.
    /// It will reset if following conditions are fulfilled:
    /// - Capture occured
    /// - Pawn has moved
    pub fifty_move_rule: u8,

    /// Stored hashes of every board state
    pub state_hashes: BTreeMap<u64, u8>,
}

impl Board {
    /// Play the next turn and update the board state.
    /// Return `State` of the previous turn. That info can be used to undo turn
    /// with an `undo` function
    pub fn next_turn(&mut self, turn: &str) -> Result<State, GameError> {
        // Moved to `play` module due to complexity
        play::next_turn(self, turn)
    }

    /// Move piece from `src` to `dst` and return captured piece if any
    /// Function caches king location in case of king movement.
    ///
    /// # Arguments
    ///
    /// * `src` - Source square
    /// * `dst` - Destination square
    pub fn move_piece(
        &mut self,
        dst: Square,
        src: Square,
    ) -> Option<(Piece, Side)> {
        let (piece, side) = self.map.remove(&src).unwrap_or_else(|| {
            panic!("Src square {} empty (dst:{})", src, dst)
        });

        // Update our cache location of the king
        if piece == Piece::King {
            self.king.insert(side, dst);
        }

        self.map.insert(dst, (piece, side))
    }

    /// Getter for a king's location
    #[inline]
    pub fn get_king_pos(&self, side: Side) -> Square {
        // Basically a shortcut function
        *self.king.get(&side).unwrap()
    }

    /// Push hash state
    pub fn hash_state_push(&mut self) -> Gamestate {
        let hash = self.calc_hash();
        let hash_cnt = self.state_hashes.entry(hash).or_insert(0);

        *hash_cnt += 1;
        match *hash_cnt {
            3 => Gamestate::DrawThreeFoldRepetition,
            _ => self.gamestate.clone(),
        }
    }

    /// Pop hash state
    ///
    /// Beware: Pop is allowed on a given state the same number of times
    /// that state was previosly pushed!
    pub fn hash_state_pop(&mut self) {
        let hash = self.calc_hash();
        *self.state_hashes.entry(hash).or_insert(0) -= 1;
    }

    /// Snapshot the board state into a hash value
    ///
    /// Hashed parts of the board are conditions used for threefold repetition
    /// rule
    fn calc_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.active_player.hash(&mut hasher);
        self.map.hash(&mut hasher);
        self.enpassant.hash(&mut hasher);
        self.castling_rights.hash(&mut hasher);

        hasher.finish()
    }

    /// Undo move based on the previous turn `State`
    ///
    /// Function does not modify active_player nor does it affect hash tree
    pub fn undo(&mut self, prev_state: State) {
        // Restore these states independently of the turn type
        self.enpassant = prev_state.enpassant;
        self.castling_rights = prev_state.castling_rights;
        self.fifty_move_rule = prev_state.fifty_move_rule;
        self.gamestate = Gamestate::Ongoing;

        // Undone turn was already parsed successfully before, so unwrap!
        match Turn::try_from(prev_state.get_turn()).unwrap() {
            Turn::Castling(turn) => self.undo_castling(turn),
            Turn::Move(turn) => self.undo_move(turn, prev_state),
        }
    }

    fn undo_castling(&mut self, turn: Castling) {
        let side = self.active_player;
        let castling_type = turn.r#type;

        let king_path = castlinginfo::get_path_king(side, castling_type);
        let rook_path = castlinginfo::get_path_rook(side, castling_type);

        // Reverse path to undo their movement
        self.move_piece(king_path.src, king_path.dst);
        self.move_piece(rook_path.src, rook_path.dst);
    }

    fn undo_move(&mut self, turn: Move, mut state: State) {
        // Move piece back to it's original location
        let src = state.moving_piece_src.expect("Source not set");
        self.move_piece(src, turn.dst);

        // Demotion if necessary
        if turn.promotion.is_some() {
            let (_, side) = self.map.get(&src).unwrap();
            self.map.insert(src, (Piece::Pawn, side));
        }

        // Restore captured piece
        if let Some((square, captured)) = state.captured.take() {
            self.map.insert(square, captured);
        }
    }
}
