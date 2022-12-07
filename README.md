# Chess turn engine
The turn engine fully implements all chess rules so it could be used to play
chess or to make chess simulations.

The engine provides a list of available and playable turns after each turn
is played. Any turn from the list can be played.
Any played turn can be undone.

Input for the engine are chess turns written in algebraic chess notation
format. But for simplicity's sake, one can use the `AvailableTurn` struct to
transform a simple layman's turn *Move piece from square A to square B* into a
chess notation turn.

## Display
The chess engine provides a basic set of tools to print out the board state in
ASCII format. Few options are available for printing out the board state.
Turn history can be printed out. Captured pieces can be printed out.
### `ViewMode::SimpleAscii` - Quite simple display format
#### Legend
   - `b`: Black player
   - `w`: White player
   - `P`: Pawn
   - `N`: Knight
   - `B`: Bishop
   - `R`: Rook
   - `K`: King
   - `Q`: Queen

 #### Example
 ```rust
 8 bR bN bB bQ bK bB bN bR
 7 bP bP bP bP bP bP bP bP
 6  -  +  -  +  -  +  -  +
 5  +  -  +  -  +  -  +  -
 4  -  +  -  +  -  +  -  +
 3  +  -  +  -  +  -  +  -
 2 wP wP wP wP wP wP wP wP
 1 wR wN wB wQ wK wB wN wR
    a  b  c  d  e  f  g  h
 ```
### `ViewMode::FancyTui` - Colorful terminal ASCII display format
Note: *colors are not visible in documentation pages*

#### Example
```rust
8  ♜  ♞  ♝  ♛  ♚  ♝  ♞  ♜
7  ♟  ♟  ♟  ♟  ♟  ♟  ♟  ♟
6
5
4
3
2  ♙  ♙  ♙  ♙  ♙  ♙  ♙  ♙
1  ♖  ♘  ♗  ♕  ♔  ♗  ♘  ♖
   a  b  c  d  e  f  g  h
```
## Example with six straightforward turns played
```rust
use chess_turn_engine::{
    ChessTurnEngine, DisplayOption, Setup, ViewMode, Gamestate
};

let mut cte = ChessTurnEngine::new(Setup::Normal).unwrap();
let turns = ["h4", "Na6", "h5", "b5", "d3", "g6"];

turns.iter().for_each(|turn| {
    cte.play_turn(turn);
});

for turn in cte.available_turns() {
    println!("{}", turn);
}

// Output:
// AvailableTurn (src: h5, dst: h6, piece: Pawn, captured: None, turn: h6
// AvailableTurn (src: h5, dst: g6, piece: Pawn, captured: Some("Pawn"), turn: hxg6
// AvailableTurn (src: d3, dst: d4, piece: Pawn, captured: None, turn: d4
// AvailableTurn (src: a2, dst: a3, piece: Pawn, captured: None, turn: a3
// AvailableTurn (src: a2, dst: a4, piece: Pawn, captured: None, turn: a4
// AvailableTurn (src: b2, dst: b3, piece: Pawn, captured: None, turn: b3
// AvailableTurn (src: b2, dst: b4, piece: Pawn, captured: None, turn: b4
// AvailableTurn (src: c2, dst: c3, piece: Pawn, captured: None, turn: c3
// AvailableTurn (src: c2, dst: c4, piece: Pawn, captured: None, turn: c4
// AvailableTurn (src: e2, dst: e3, piece: Pawn, captured: None, turn: e3
// AvailableTurn (src: e2, dst: e4, piece: Pawn, captured: None, turn: e4
// AvailableTurn (src: f2, dst: f3, piece: Pawn, captured: None, turn: f3
// AvailableTurn (src: f2, dst: f4, piece: Pawn, captured: None, turn: f4
// AvailableTurn (src: g2, dst: g3, piece: Pawn, captured: None, turn: g3
// AvailableTurn (src: g2, dst: g4, piece: Pawn, captured: None, turn: g4
// AvailableTurn (src: b1, dst: c3, piece: Knight, captured: None, turn: Nc3
// AvailableTurn (src: b1, dst: a3, piece: Knight, captured: None, turn: Na3
// AvailableTurn (src: b1, dst: d2, piece: Knight, captured: None, turn: Nd2
// AvailableTurn (src: c1, dst: d2, piece: Bishop, captured: None, turn: Bd2
// AvailableTurn (src: c1, dst: e3, piece: Bishop, captured: None, turn: Be3
// AvailableTurn (src: c1, dst: f4, piece: Bishop, captured: None, turn: Bf4
// AvailableTurn (src: c1, dst: g5, piece: Bishop, captured: None, turn: Bg5
// AvailableTurn (src: c1, dst: h6, piece: Bishop, captured: None, turn: Bh6
// AvailableTurn (src: d1, dst: d2, piece: Queen, captured: None, turn: Qd2
// AvailableTurn (src: e1, dst: d2, piece: King, captured: None, turn: Kd2
// AvailableTurn (src: g1, dst: h3, piece: Knight, captured: None, turn: Nh3
// AvailableTurn (src: g1, dst: f3, piece: Knight, captured: None, turn: Nf3
// AvailableTurn (src: h1, dst: h2, piece: Rook, captured: None, turn: Rh2
// AvailableTurn (src: h1, dst: h3, piece: Rook, captured: None, turn: Rh3
// AvailableTurn (src: h1, dst: h4, piece: Rook, captured: None, turn: Rh4
```
## Example with two turns played
```rust
let mut cte = ChessTurnEngine::new(Setup::Normal).unwrap();
assert!(cte.play_turn("d4").is_ok()); // Play "d4", starting turn

// Play a random turn for the black player
let mut next_random_turn =
    String::from(cte.available_turns().first().unwrap().get_turn());
assert!(cte.play_turn(&next_random_turn).is_ok());

// Play "a3" turn using only info about the source and destination squares
for turn in cte.available_turns() {
    if turn.src != String::from("a2") && turn.dst != String::from("a3") {
        continue;
    }
    next_random_turn = String::from(turn.get_turn());
    break;
}
assert!(cte.play_turn(&next_random_turn).is_ok());
assert_eq!(cte.gamestate(), Gamestate::Ongoing);

cte.display_on_screen(DisplayOption::BoardView(ViewMode::SimpleAscii));
// 8 bR bN bB bQ bK bB bN bR
// 7 bP bP bP  - bP bP bP bP
// 6  -  +  -  +  -  +  -  +
// 5  +  -  + bP  +  -  +  -
// 4  -  +  - wP  -  +  -  +
// 3 wP  -  +  -  +  -  +  -
// 2  - wP wP  + wP wP wP wP
// 1 wR wN wB wQ wK wB wN wR
//    a  b  c  d  e  f  g  h
```

# About
I don't really play chess that much, but I needed something to practice my Rust
skills, so this was sort of a fun project.
This particular crate was created for the purpose of [Pacifist chess
simulation](https://github.com/Rqnsom/pacifist-chess-simulation).
