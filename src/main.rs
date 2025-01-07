use std::io;
use std::iter;
use std::usize;

const GAME_LIST: [&'static str; 2] = ["Tic Tac Toe", "Chess"];

fn main() {
    println!("Welcome, what game do you wish to play now?");
    for (index, game) in GAME_LIST.iter().enumerate() {
        println!("for {game} press {index}");
    }
    let mut game_chosen = String::new();
    io::stdin()
        .read_line(&mut game_chosen)
        .expect("Failed to read line");
    let val_check: i8 = game_chosen.trim().parse().expect("Should pass number");
    match val_check {
        0 => {
            println!("Tic Tac Toe");
        }
        1 => {
            println!("Not yet avalaible, coming soon");
        }
        _ => {
            println!("You lost");
        }
    };
}

struct Play {
    l: usize,
    c: usize,
}

impl Play {
    fn new(l: usize, c: usize) -> Play {
        Play { l, c }
    }
}

struct TicTacToe {
    quadrants: Vec<i8>,
    dimensions: usize,
}

impl TicTacToe {
    fn new() -> TicTacToe {
        TicTacToe {
            quadrants: vec![0; 9],
            dimensions: 3,
        }
    }

    fn map_play(play: &str) -> (usize, usize) {
        let (l, n) = play.split_at(1);
        let (mapped_l, mapped_n): (usize, usize);
        match l {
            "A" => {
                mapped_l = 0;
            }
            "B" => {
                mapped_l = 1;
            }
            "C" => {
                mapped_l = 2;
            }
            _ => {
                mapped_l = 999;
            }
        };
        match n {
            "1" => {
                mapped_n = 0;
            }
            "2" => {
                mapped_n = 1;
            }
            "3" => {
                mapped_n = 2;
            }
            _ => {
                mapped_n = 999;
            }
        };
        (mapped_l, mapped_n)
    }

    fn check_play(game: &mut TicTacToe, player: i8, l: usize, c: usize) -> bool {
        if game.quadrants[game.dimensions * l + c] == 0 {
            game.quadrants[game.dimensions * l + c] = player;
            return true;
        }
        false
    }

    /*     0   1   2
        0 q00 q01 q02
        1 q10 q11 q12
        2 q20 q21 q22

        starting from 0:
            if on main diagonal the subtraction of indexes equals 0.
            if on secondary diagonal the addition of indexes equals dimensions.

        after q22 placed:
            check all in same line
            check all in same column
            check all in main diagonal
            check all in secondary diagonal
    */
    fn check_if_win(game: &TicTacToe, play: &Play, player: i8) -> bool {
        let mut counter: i8 = 0;
        let mut tmp_j: usize = game.dimensions;

        // First, check if it's true on the secondary diagonal
        if play.l + play.c == game.dimensions {
            for i in 0..game.dimensions {
                counter += game.quadrants[game.dimensions * i + tmp_j];
                tmp_j -= tmp_j;
            }
            if counter == player * (game.dimensions as i8 + 1) {
                return true;
            }
            counter = 0;
        }

        // Check if it's true on the main diagonal
        if play.l + play.c == game.dimensions {
            for i in 0..game.dimensions {
                counter += game.quadrants[game.dimensions * i + i];
            }
            if counter == player * (game.dimensions as i8 + 1) {
                return true;
            }
            counter = 0;
        }

        // Check if it's true on the same line
        for i in 0..game.dimensions {
            counter += game.quadrants[game.dimensions * play.l + i];
        }
        if counter == player * (game.dimensions as i8 + 1) {
            return true;
        }
        counter = 0;

        // Check if it's true on the same column
        for i in 0..game.dimensions {
            counter += game.quadrants[game.dimensions * i + play.c];
        }
        if counter == player * (game.dimensions as i8 + 1) {
            return true;
        }

        false
    }
}
