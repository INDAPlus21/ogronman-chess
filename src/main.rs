use crate::lib::Game;
use std::io;
use std::io::BufRead;
mod lib;


fn main() {
    let mut game = Game::new();

    game.init_board();

    game.print_board();
    println!("Current turn is:");
    println!("{:#?}", game.get_game_turn());

    
    let input = io::stdin();
    
    loop{
        let lines = input.lock().lines().next().unwrap().unwrap();

        let mut pos:Vec<char> = lines.chars().collect();

        if pos.len() == 5{
            let mut _from:String = String::from("");
            _from.push(pos[0]);
            _from.push(pos[1]);
            let mut _to:String = String::from("");
            _to.push(pos[3]);
            _to.push(pos[4]);
    
            game.make_move(_from, _to);
            game.print_board();
            println!("Current turn is:");
            println!("{:#?}", game.get_game_turn());
        }
    }

}
