# ogronman-chess


| **Function**      | **Description** |
| ----------- | ----------- |
| `pub fn init_board(&mut self) -> ()`  | Initialises a new board with pieces   |
| `pub fn make_move(&mut self, _from: String, _to: String) -> ()`   | Moves the given piece to the given posistion, if the game is in check the move is only legal if the game after the move is no longer in check. If the move is illegal nothing happens      |
| `pub fn set_promotion(&mut self, _piece_pos: u8, promotion_piece:String) -> ()`  | Promotes a pawn to the given unit   |
| `pub fn change_turn(&mut self) -> ()`  | Changes the turn  |


The program also uses an enumerable `GameState` with the values:

- `InProgress`,
- `Check`,
- `Checkmate`
- `GameOver`


As well the enumerable `ColorTurn` with the values:
- `White`,
- `Black`,

The library also contains a really simple ai that can make moves for the black player and the black player only

If you call the function `pub fn make_ai_move(&mut self) -> ()` the ai will make one completely random, but legal move

Functions that are not described in this file are probably self explanatory

good luck

xoxo