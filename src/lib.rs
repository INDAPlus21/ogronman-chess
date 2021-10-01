use std::fmt;
use std::cmp;
use std::io;
use std::io::prelude::*;    
use rand::Rng;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ColorTurn{
    White,
    Black
}

/** 
     * 
     *     
     * 
    */

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */
pub struct Game {
    /* save board, active colour, ... */
    board: Vec<Vec<u8>>,
    move_offset: Vec<i8>,
    move_offset_knight: Vec<i8>,
    move_to_edge: Vec<Vec<u8>>,
    state: GameState,
    turn: ColorTurn,
    last_moved_pawn: Vec<u8>,

}

const _NONE:u8 = 0;
const _PAWN:u8 = 1;
const _BISHOP:u8 = 2;
const _KNIGHT:u8 = 3;
const _ROOK:u8 = 4;
const _QUEEN:u8 = 5;
const _KING:u8 = 6;
const _WHITE:u8 = 8;
const _BLACK:u8 = 16;

//const _MOVEOFFSET:Vec<i8> = vec![8, -8, -1, 1, 7 -7, 9, -9];

const _STARTFEN:&str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

impl Game {


    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            
            /* initialise board, set active colour to white, and sets up all of the move offsets */
            state: GameState::InProgress,
            turn: ColorTurn::White,
            board: Vec::with_capacity(64),
            move_offset: vec![8, -8, -1, 1, 7, -7, 9, -9],
            move_offset_knight: vec![-15, -17, -6, -10, 10, 6, 17, 15],
            move_to_edge: Vec::with_capacity(64),
            last_moved_pawn: Vec::with_capacity(2),
        }

    }


    /**
     * A function that takes in a start value (the posistion the piece has before the move)
     * if the piece can not move any further (since it is a pawn), the set_promotion function is called
     */
    fn should_promote (&mut self, _start: u8) -> () {
        if self.board[_start as usize][0] == _PAWN && self.board[_start as usize][1] == _WHITE {
            if _start+8*2 > 64 {
                self.set_promotion(_start);
            }
        }else if self.board[_start as usize][0] == _PAWN && self.board[_start as usize][1] == _BLACK{
            if _start-8*2 < 0 {
                self.set_promotion(_start);
            }
        }
    }

    /**
     * A function that generates all of the possible moves that the pawns can move
     * 
     * Returns a vector with all of the possible moves with the structure [start, end]
     */
    fn generate_short_moves(&self, _start: u8) -> Vec<Vec<u8>> {
        let start_index = 0;
        let end_index = 8;

        let mut possible_moves:Vec<Vec<u8>> = Vec::new(); 

        if self.board[_start as usize][0] == _PAWN && self.board[_start as usize][1] == _WHITE {
            if _start+8 < 64 && self.board[(_start+8) as usize][0] == _NONE{
                possible_moves.push(vec![_start, _start+8]);
            }
            if self.board[(_start+7) as usize][1] != self.board[_start as usize][1] && self.board[(_start+7) as usize][0] != _NONE {
                possible_moves.push(vec![_start, _start+7]);
            }
            if self.board[(_start+9) as usize][1] != self.board[_start as usize][1] && self.board[(_start+9) as usize][0] != _NONE {
                possible_moves.push(vec![_start, _start+9]);
            }
        }else if self.board[_start as usize][0] == _PAWN && self.board[_start as usize][1] == _BLACK {
            if _start-8 > 0 && self.board[(_start-8) as usize][0] == _NONE{
                possible_moves.push(vec![_start, _start-8]);
            }
            if self.board[(_start-7) as usize][1] != self.board[_start as usize][1] && self.board[(_start-7) as usize][0] != _NONE {
                possible_moves.push(vec![_start, _start-7]);
            }
            if self.board[(_start-9) as usize][1] != self.board[_start as usize][1] && self.board[(_start-9) as usize][0] != _NONE {
                possible_moves.push(vec![_start, _start-9]);
            }
        }

        // if pawn is in startposistion it can move two spaces forward
        if self.board[_start as usize][1] == _WHITE && _start >= 8 && _start < 16 && self.board[(_start+2*8) as usize][0] == _NONE{
            possible_moves.push(vec![_start, _start+2*8]);
        }else if self.board[_start as usize][1] == _BLACK && _start >= 46 && _start < 55 && self.board[(_start-2*8) as usize][0] == _NONE{
            possible_moves.push(vec![_start, _start-2*8]);
        }

        // add en passante moves
        if self.last_moved_pawn[1] != 2 {
            if self.last_moved_pawn[1] != self.board[_start as usize][1] {
                if self.board[_start as usize][1] == _WHITE {
                    possible_moves.push(vec![_start, self.last_moved_pawn[0]+8]);
                }else{
                    possible_moves.push(vec![_start, self.last_moved_pawn[0]-8]);
                }
            }
        }
        return possible_moves;
    }

    /**
     * Generate all of the moves that a knight at the start posistion can move
     * 
     * Returns a vector with all of the possible moves with the structure [start, end]
     */
    fn generate_knight_moves(&self, _start: u8) -> Vec<Vec<u8>> {
        let mut possible_moves:Vec<Vec<u8>> = Vec::new(); 

        for i in 0..8 {
            if _start as i8 + self.move_offset_knight[i] >= 0 && _start as i8 + self.move_offset_knight[i] <= 64 {
                let _target:u8 = (_start as i8 + self.move_offset_knight[i]) as u8;
                if self.board[_target as usize][1] != self.board[_start as usize][1] {
                    possible_moves.push(vec![_start, (_target)]);
                }
            }
        }
        return possible_moves;

    }
    /**
     * Generates all the moves a king at a given posistion (_start) can move
     * 
     * Returns a vector with all of the possible moves with the structure [start, end]
     */
    fn generate_king_moves(&self, _start: u8) -> Vec<Vec<u8>> {

        let mut possible_moves:Vec<Vec<u8>> = Vec::new(); 

        for _dir_index in 0..8{
            for mut _n in 0..self.move_to_edge[_start as usize][_dir_index]{
                let _target:u8 = (_start as i8 + (self.move_offset[_dir_index] * (_n+1) as i8)) as u8;
                
                if _target > 0 && _target < 65 {
                    //If blocked by friendly
                    if self.board[_target as usize][0] != _NONE && self.board[_target as usize][1] == self.board[_start as usize][1] {
                        break;
                    }

                    possible_moves.push(vec![_start, _target]);

                    if self.board[_target as usize][1] != self.board[_start as usize][1] {
                        _n = self.move_to_edge[_start as usize][_dir_index];
                    }
                }else{
                    _n = self.move_to_edge[_start as usize][_dir_index];
                }
                break;
            }
        }

        return possible_moves;

    }
    
    /**
     * Generates all of the moves a sliding piece can move from a given posistion
     * 
     * Returns a vector with all of the possible moves with the structure [start, end]
     */
    fn generate_long_moves(&self, _start: u8) -> Vec<Vec<u8>>{
        let mut start_index = 0;
        let mut end_index = 8;
        if self.board[_start as usize][0] == _BISHOP {
            start_index = 4;
        }else if self.board[_start as usize][0] == _ROOK {
            end_index = 4;
        }

        let mut possible_moves:Vec<Vec<u8>> = Vec::new(); 
        for _dir_index in start_index..end_index{
            for mut _n in 0..self.move_to_edge[_start as usize][_dir_index]{
                let _target:u8 = (_start as i8 + (self.move_offset[_dir_index] * (_n+1) as i8)) as u8;
                if _target > 0 && _target < 65 {
                    //If blocked by friendly
                    if self.board[_target as usize][0] != _NONE && self.board[_target as usize][1] == self.board[_start as usize][1] {
                        break;
                    }

                    possible_moves.push(vec![_start, _target]);

                    if self.board[_target as usize][1] != self.board[_start as usize][1] {
                        _n = self.move_to_edge[_start as usize][_dir_index];
                    }
                }else{
                    _n = self.move_to_edge[_start as usize][_dir_index];
                }

            }
        }
        return possible_moves;
    }


    /**
     * Takes a fen string (a specific way to structure a chess board)
     * and creates / initialises the board from the fen string
     * 
     * input
     */
    pub fn load_fen_board(&mut self,fen_string: String ) -> (){
        
        let mut file:usize = 0;
        let mut rank:usize = 7;

        for c in fen_string.chars(){
            if c == '/'{
                file = 0;
                rank -= 1;
            }else{
                if c.is_numeric() {
                    file += c as usize;
                }else{
                    let mut piece_color = _BLACK;
                    if c.is_uppercase() {
                        piece_color = _WHITE; 
                    }

                    let piece_type = Game::piece_from_symbol(c.to_ascii_lowercase());
                    self.board[rank*8+file] = vec![piece_type, piece_color];
                    file += 1;
                }
            }
        }

    }
    

    /**
     * takes a char (in this case p,n,b,r,q,k) representing each kind of piece
     * then converts that char to a u8 that represents each piece
     */
    fn piece_from_symbol(c:char) -> u8 {
        let mut _s = c.to_string();
        _s = _s.chars().map(|_s| match _s {      
            'p' => _PAWN.to_string(),  //Game::_PAWN
            'n' => _KNIGHT.to_string(), 
            'b' => _BISHOP.to_string(),
            'r' => _ROOK.to_string(),
            'q' => _QUEEN.to_string(),
            'k' => _KING.to_string(),
            _ => _NONE.to_string()
        }).collect();
        let piece:u8 = _s.parse::<u8>().unwrap(); //Gör om bokstäver till siffror som kan motsvara till brädet t.ex.

        return piece;
    }


    /**
     * A function that makes the black player move a random piece that can move
     * to a random possible position 
     */
    pub fn make_ai_move(&mut self) -> (){

        println!("now ai is making move");

        let mut all_moves:Vec<Vec<u8>> = Vec::new();
        let mut good_moves:Vec<Vec<u8>> = Vec::new();
        self.state = GameState::Check;
        for _in in 0..64{
            if self.is_king_check() == false {
                if self.board[_in as usize][0] == _BISHOP || self.board[_in as usize][0] == _ROOK || self.board[_in as usize][0] == _QUEEN {
                    all_moves.append(&mut self.generate_long_moves(_in));
                } else if self.board[_in as usize][0] == _PAWN{
                    all_moves.append(&mut self.generate_short_moves(_in));
                } else if self.board[_in as usize][0] == _KING{
                    all_moves.append(&mut self.generate_king_moves(_in));
                } else if self.board[_in as usize][0] == _KNIGHT{
                    all_moves.append(&mut self.generate_knight_moves(_in));
                }
            }
        }

        good_moves.append(&mut self.get_good_moves(&all_moves));

        println!("Have calculated all moves");

        if self.is_king_check() == false {
            self.state = GameState::InProgress;
        }

        let mut rng = rand::thread_rng();

        if all_moves.len() == 0 {
            
            self.state = GameState::GameOver;
            return;

        }else{
            if good_moves.len() == 0 {
                
                let len:u8 = (all_moves.len()-1) as u8;


                loop{            
        
                    let _rand: u8 = rng.gen_range(0..len);
        
                    
        
                    let _from:u8 = all_moves[_rand as usize][0];
                    let _to:u8 = all_moves[_rand as usize][1];
                    if all_moves.contains(&all_moves[_rand as usize]) && self.board[_from as usize][1] == _BLACK{
                        self.board[_to as usize][0] = self.board[_from as usize][0];
                        self.board[_to as usize][1] = self.board[_from as usize][1];
                        self.board[_from as usize] = vec![_NONE, 2];
            
                        self.print_board();
                        
                        if self.is_king_check() == true{
            
                            println!("Kungen är i shack");
                            println!("Kungen är i shack");
                            self.state = GameState::Check;
                        }
            
                        self.change_turn();
                        break;
                    }
                } 
            }

        }

    }

    fn get_good_moves(&mut self, possible_moves: &Vec<Vec<u8>>) -> Vec<Vec<u8>>{
        let mut good_moves:Vec<Vec<u8>> = Vec::new();
        for _i in 0..possible_moves.len(){
            if self.board[possible_moves[_i as usize][0]][1] == _BLACK {
                if self.board[possible_moves[_i as usize][1]][1] == _WHITE {
                    good_moves.push(possible_moves[_i]);
                }
            }
        }
        return good_moves;
    }


    /**
     * Initialises the board and calls the load_fen_board function
     * As well as calls the get_edge function
     */
    pub fn init_board(&mut self) -> (){
        for _i in 0..self.board.capacity(){
            self.board.push(Vec::new());
            
            self.board[_i] = vec![_NONE, 2];
        }
        self.last_moved_pawn = vec![65, 2];  
        Game::load_fen_board(self, _STARTFEN.to_string());
        Game::get_edge(self);
        
    }

    /**
     * A function that calculates the length/distance from each square on the board
     * To the edge of the board and stores those values in the vector move_to_edge
     */
    fn get_edge(&mut self) -> (){
        for _i in 0..self.move_to_edge.capacity(){
            self.move_to_edge.push(Vec::new());
        }

        for file in 0..8{
            for rank in 0..8 {
                let north:u8 = 7-rank;
                let south:u8 = rank;
                let west:u8 = file;
                let east:u8 = 7- file;

                let _index = rank*8 + file;
                self.move_to_edge[_index as usize] = vec![north, south, west, east, cmp::min(north, west), cmp::min(south, east), cmp::min(north, east), cmp::min(south, west)];
            }
        }
    }


    /**
     * A function that first calculates all of the possible_moves depending on the start posistion
     * then if the game is in progress if the move is legal, moves the piece to the _to posistion
     * If the game is in check the move is only legal if the resulting move results in the game no longer 
     * being in check
     */
    pub fn make_move(&mut self, _from: String, _to: String) -> () {
        
        //Get all the moves
        let _from_pos:u8 = self.pos_to_int(_from);
        let _to_pos:u8 = self.pos_to_int(_to);
        let mut possible_moves:Vec<Vec<u8>> = Vec::new();
        if self.board[_from_pos as usize][0] == _BISHOP || self.board[_from_pos as usize][0] == _ROOK || self.board[_from_pos as usize][0] == _QUEEN {
            possible_moves.append(&mut Game::generate_long_moves(self, _from_pos));
        } else if self.board[_from_pos as usize][0] == _PAWN{
            possible_moves.append(&mut Game::generate_short_moves(self, _from_pos));
            self.should_promote(_from_pos);
        } else if self.board[_from_pos as usize][0] == _KING{
            possible_moves.append(&mut Game::generate_king_moves(self, _from_pos));
        } else if self.board[_from_pos as usize][0] == _KNIGHT{
            possible_moves.append(&mut Game::generate_knight_moves(self, _from_pos));
        }

        //If the game is inprogess
        if self.state == GameState::InProgress {
            //If move is legal, move piece
            if Game::is_move_legal(self, possible_moves, _from_pos, _to_pos) {


                if self.board[_from_pos as usize][0] == _PAWN {
                    let _enPassante:bool = self.should_enpassante(_from_pos, _to_pos);

                    if  self.last_moved_pawn[1] != self.board[_from_pos as usize][1] && self.last_moved_pawn[1] != 2 {
                        if self.board[_from_pos as usize][1] == _WHITE {
                            if self.last_moved_pawn[0]+8 == _to_pos {
                                self.board[self.last_moved_pawn[0] as usize] = vec![_NONE, 2];
                            }
                        }else{
                            if self.last_moved_pawn[0]-8 == _to_pos {
                                self.board[self.last_moved_pawn[0] as usize] = vec![_NONE, 2];
                            }
                        }
                    }
                }


                self.board[_to_pos as usize][0] = self.board[_from_pos as usize][0];
                self.board[_to_pos as usize][1] = self.board[_from_pos as usize][1];
                self.board[_from_pos as usize] = vec![_NONE, 2];

                self.print_board();
                //If it results in check, change gamestate to check
                if self.is_king_check() == true{

                    println!("Kungen är i shack");
                    println!("Kungen är i shack");
                    self.state = GameState::Check;
                }

                self.change_turn();

            }


      
        }else if self.state == GameState::Check{

            if Game::is_move_legal(self, possible_moves, _from_pos, _to_pos){
                let mut temp_vec:Vec<Vec<u8>> = Vec::new();
                temp_vec.push(vec![65,65]);
                temp_vec.push(vec![65,65]);
                temp_vec[0][0] = self.board[_from_pos as usize][0];
                temp_vec[0][1] = self.board[_from_pos as usize][1];
                temp_vec[1][0] = self.board[_to_pos as usize][0];
                temp_vec[1][1] = self.board[_to_pos as usize][1];

                self.board[_to_pos as usize][0] = self.board[_from_pos as usize][0];
                self.board[_to_pos as usize][1] = self.board[_from_pos as usize][1];
                self.board[_from_pos as usize] = vec![_NONE, 2];

                if self.is_king_check() == true{
                    self.board[_to_pos as usize][0] = temp_vec[1][0];
                    self.board[_to_pos as usize][1] = temp_vec[1][1];
                    self.board[_from_pos as usize][0] = temp_vec[0][0];
                    self.board[_from_pos as usize][1] = temp_vec[0][1];
                }else{
                    
                    self.change_turn();
                    self.print_board();
                    self.state = GameState::InProgress;
                }
            }else {
            }
        }
        //return self;
    }

    fn should_enpassante(&mut self, _from:u8, _to:u8) -> bool{
        if self.board[_from as usize][1] == _WHITE {
            if _from+8*2 == _to {
                self.last_moved_pawn[0] = _to;
                self.last_moved_pawn[1] = _WHITE;
                return true;
            }
        }else{
            if _from-8*2 == _to {
                self.last_moved_pawn[0] = _to;
                self.last_moved_pawn[1] = _BLACK;
                return true;
            }
        }
        return false;
    }

    
    /**
     * A function that changes a string of the posistion for example "a4"
     * To the corresponding index of the board vector
     */
    pub fn pos_to_int(&mut self, _in: String) -> u8{

        let mut _c:String = _in.chars().nth(0).unwrap().to_string();
        _c = _c.chars().map(|_c| match _c {      // t.ex. a blir 0
                'a' => "1", 
                'b' => "2", 
                'c' => "3",
                'd' => "4",
                'e' => "5",
                'f' => "6",
                'g' => "7",
                'h' => "8",
                _ => "1"
            }).collect();

        let mut _s:String  = _in.chars().nth(1).unwrap().to_string();

        _s = _s.chars().map(|_s| match _s {      // t.ex. a blir 0
            '1' => "0", 
            '2' => "8", 
            '3' => "16",
            '4' => "24",
            '5' => "32",
            '6' => "40",
            '7' => "48",
            '8' => "56",
            _ => "0"
        }).collect();
        let _pos1:u8 = _c.parse::<u8>().unwrap();
        let _pos2:u8 = _s.parse::<u8>().unwrap();
        return _pos1+_pos2-1;
    }


    /**
     * A function that checks if the king is in check
     * 
     * The function does this my calculating all of the possible moves on the board
     * if there is a move between one piece and the opponents king, then the game is in check
     */
    fn is_king_check(&self) -> bool {

        let mut all_moves:Vec<Vec<u8>> = Vec::new();
        let mut opponent_king:u8 = 0;
        for _in in 0..64{
            if self.board[_in as usize][0] == _BISHOP || self.board[_in as usize][0] == _ROOK || self.board[_in as usize][0] == _QUEEN {
                all_moves.append(&mut self.generate_long_moves(_in));
            } else if self.board[_in as usize][0] == _PAWN{
                all_moves.append(&mut self.generate_short_moves(_in));
            } else if self.board[_in as usize][0] == _KING{
                all_moves.append(&mut self.generate_king_moves(_in));
            } else if self.board[_in as usize][0] == _KNIGHT{
                all_moves.append(&mut self.generate_knight_moves(_in));
            }
            
            if self.state == GameState::InProgress {
                if self.board[_in as usize][0] == _KING && self.board[_in as usize][1] == _BLACK && self.turn == ColorTurn::White{
                    opponent_king = _in;
                }else if self.board[_in as usize][0] == _KING && self.board[_in as usize][1] == _WHITE && self.turn == ColorTurn::Black{
                    opponent_king = _in;
                }
            } else{
                if self.board[_in as usize][0] == _KING && self.board[_in as usize][1] == _BLACK && self.turn == ColorTurn::Black{
                    opponent_king = _in;
                }else if self.board[_in as usize][0] == _KING && self.board[_in as usize][1] == _WHITE && self.turn == ColorTurn::White{
                    opponent_king = _in;
                }
            }


        }

        let mut is_check:bool = false;


        for _n in 0..64{
            let wanted_move:Vec<u8> = vec![_n, opponent_king];
            if all_moves.contains(&wanted_move) && (self.board[_n as usize][1] != self.board[opponent_king as usize][1]) {
                is_check = true;
            }
        }

        return is_check;
    }


    /**
     * Check if move is legal
     */
    pub fn is_move_legal(&self, possible_moves: Vec<Vec<u8>>, _from:u8, _to:u8) -> bool{
        let wanted_move:Vec<u8> = vec![_from, _to];
        if possible_moves.contains(&wanted_move) && ((self.board[_from as usize][1] == _WHITE && self.turn == ColorTurn::White) || (self.board[_from as usize][1] == _BLACK && self.turn == ColorTurn::Black)) {
            return true;
        }else{
            return false;
        }
    }

    /**
     * Promotes a pawn to a piecec of the players choice
     */
    pub fn set_promotion(&mut self, _piece_pos: u8) -> () {
        println!("Promote your pawn");
        println!("Print Q for queen");
        println!("Print K for king");
        println!("Print R for rook");
        println!("Print B for bishop");

        let input = io::stdin();


        let mut lines = input.lock().lines().next().unwrap().unwrap();

        let mut promotion_piece = lines.to_ascii_lowercase().chars().nth(0).unwrap().to_string();
        println!("Promotion piece = {}", promotion_piece);

        promotion_piece = promotion_piece.chars().map(|promotion_piece| match promotion_piece {      // t.ex. a blir 0
            'q' => _QUEEN.to_string(), 
            'k' => _KING.to_string(), 
            'r' => _ROOK.to_string(),
            'b' => _BISHOP.to_string(),
            _ => _QUEEN.to_string()
        }).collect();

        self.board[_piece_pos as usize][0] = promotion_piece.parse::<u8>().unwrap();

    }

    /**
     * Changes the turn
     */
    pub fn change_turn(&mut self) -> (){
        if self.turn == ColorTurn::White {
            if self.last_moved_pawn[1] == _BLACK {
                self.last_moved_pawn = vec![_NONE, 2];
            }
            self.turn = ColorTurn::Black;
        }else{
            if self.last_moved_pawn[1] == _WHITE {
                self.last_moved_pawn = vec![_NONE, 2];
            }
            self.turn = ColorTurn::White;
        }
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    /// get the current game turn.
    pub fn get_game_turn(&self) -> ColorTurn{
        self.turn
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling.
    /// 
    /// Not used at the moment since all of it is calculated in make_move and other functions
    pub fn get_possible_moves(&self, _postion: String) -> Option<Vec<String>> {
        None
    }

    /**
     * Gets a piece value and converts it to the corresponding chess piece char
     * For example
     * Pawn is p and so on.
     */
    fn piece_to_char(_piece: u8, _color: u8) -> String {
        let mut return_string = " ".to_string();
        match _piece {
            _NONE => return_string.push_str(" * "),
            _PAWN => return_string.push_str("P"),
            _KNIGHT => return_string.push_str("Kn"),
            _BISHOP => return_string.push_str("B"),
            _ROOK => return_string.push_str("R"),
            _QUEEN => return_string.push_str("Q"),
            _KING => return_string.push_str("K"),
            _ => return_string.push_str(" * ")
        }
        if _color == _WHITE {
            return_string.push_str("w ");
        }else if _color == _BLACK {
            return_string.push_str("b ");
        }
        return return_string;
    }
    /// Prints the board in the console
    pub fn print_board(&self) -> (){
        let mut print_board:String = String::new();
        print_board += "   a   b   c   d   e   f   g   h";
        let mut num:u8 = 8;
        for n in 0..self.board.len(){
            if n % 8 == 0 {
                print_board += "\n";
                print_board += &((n/8+1).to_string());
                num -= 1;
            }
            //println!("{}", print_board);
            print_board += &Game::piece_to_char(self.board[n][0], self.board[n][1]);
        }
        print!("\x1B[2J");
        println!("{}", print_board);
    }
}

/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        
        write!(f, "")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    
    //
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let mut game = Game::new();
        println!("{:?}", game);
        println!("{:#?}", game.board);
        println!("{:?}", game.get_game_turn());

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }


    #[test]
    fn does_moves_work() {

        let mut game = Game::new();
        println!("{:?}", game);
        println!("{:#?}", game.board);
        println!("{:?}", game.get_game_turn());
        println!("{:?}", game.get_game_turn());
        game.init_board();
        println!("Printing board");
        game.print_board();
        game.make_move("a2".to_string(), "a4".to_string());
        println!("{:?}", game.get_game_turn());
        game.make_move("d7".to_string(), "d5".to_string());
        println!("{:?}", game.get_game_turn());
        game.make_move("a1".to_string(), "a3".to_string());
        println!("{:?}", game.get_game_turn());
        game.make_move("c8".to_string(), "e6".to_string());
        println!("{:?}", game.get_game_turn());

        //Can only move knight if all other moves have gone through
        game.make_move("g1".to_string(), "h3".to_string());
        println!("{:?}", game.get_game_turn());
        //Knight == 3
        assert_eq!(game.board[23][0], 3);
        assert_eq!(game.board[24][0], 1);
    }

    #[test]
    fn does_check_work() {

        let mut game = Game::new();
        game.init_board();
        game.load_fen_board("rnbqqbnr/pppppkpp/8/8/8/8/PPPPPRPP/RNBQKBNR".to_string());
        game.print_board();
        game.make_move("f2".to_string(), "f4".to_string());
        println!("{:?}", game.get_game_turn());

        assert_eq!(game.get_game_state(), GameState::Check);

    }


    
    #[test]
    fn can_check_move(){
        let mut game = Game::new();
        game.init_board();
        game.load_fen_board("rnbqqbnr/pppppkpp/8/8/8/8/PPPPPRPP/RNBQKBNR".to_string());
        game.print_board();
        game.make_move("f2".to_string(), "f4".to_string());
        println!("{:?}", game.get_game_turn());
        game.make_move("f7".to_string(), "f6".to_string());
        println!("{:?}", game.get_game_turn());
        println!("Could not move there since is in check");
        game.make_move("f7".to_string(), "g6".to_string());
        println!("{:?}", game.get_game_turn());

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }


    #[test]
    fn does_en_passante_work(){
        let mut game = Game::new();
        game.init_board();
        game.load_fen_board("rnbqkbnr/pppppppp/8/pppppppp/8/8/PPPPPPPP/RNBQKBNR".to_string());
        game.print_board();
        game.make_move("a2".to_string(), "a3".to_string());
        game.print_board();
        println!("{:?}", game.get_game_turn());
        game.make_move("b5".to_string(), "b4".to_string());
        game.print_board();
        println!("{:?}", game.get_game_turn());
        game.make_move("c2".to_string(), "c4".to_string());
        game.print_board();
        println!("{:?}", game.get_game_turn());
        game.make_move("b4".to_string(), "c3".to_string());
        game.print_board();
        println!("{:?}", game.get_game_turn());

        assert_eq!(game.board[18][1], 16);
        assert_eq!(game.board[26][0], 0);
    }


}