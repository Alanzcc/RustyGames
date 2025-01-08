use std::io;
use std::usize;

fn main() {
    println!("Welcome, what game do you wish to play now?");
    let game_list = ["Tic Tac Toe", "Chess"];
    for (index, game) in game_list.iter().enumerate() {
        println!("for {game} press {index}");
    }
    let mut self_chosen = String::new();
    io::stdin()
        .read_line(&mut self_chosen)
        .expect("Failed to read line");
    let val_check: i8 = self_chosen.trim().parse().expect("Should pass number");
    match val_check {
        0 => {
            println!("Tic Tac Toe");
        }
        1 => {
            println!("Not yet avalaible, coming soon");
            return;
        }
        _ => {
            println!("You already lost");
            return;
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

    fn map_play(play: &str) -> Play {
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
                mapped_l = 99;
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
                mapped_n = 99;
            }
        };
        Play::new(mapped_l, mapped_n)
    }

    fn check_play(&mut self, player: i8, play: &Play) -> bool {
        if self.quadrants[self.dimensions * play.l + play.c] == 0 {
            self.quadrants[self.dimensions * play.l + play.c] = player;
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
    fn check_if_win(&self, play: &Play, player: i8) -> bool {
        let mut counter: i8 = 0;
        let mut tmp_j: usize = self.dimensions;

        // First, check if it's true on the secondary diagonal
        if play.l + play.c == self.dimensions {
            for i in 0..self.dimensions {
                counter += self.quadrants[self.dimensions * i + tmp_j];
                tmp_j -= tmp_j;
            }
            if counter == player * (self.dimensions as i8 + 1) {
                return true;
            }
            counter = 0;
        }

        // Check if it's true on the main diagonal
        if play.l + play.c == self.dimensions {
            for i in 0..self.dimensions {
                counter += self.quadrants[self.dimensions * i + i];
            }
            if counter == player * (self.dimensions as i8 + 1) {
                return true;
            }
            counter = 0;
        }

        // Check if it's true on the same line
        for i in 0..self.dimensions {
            counter += self.quadrants[self.dimensions * play.l + i];
        }
        if counter == player * (self.dimensions as i8 + 1) {
            return true;
        }
        counter = 0;

        // Check if it's true on the same column
        for i in 0..self.dimensions {
            counter += self.quadrants[self.dimensions * i + play.c];
        }
        if counter == player * (self.dimensions as i8 + 1) {
            return true;
        }

        false
    }

    fn next_player(player: i8) -> &'static str {
        match player {
            1 => "X",
            -1 => "O",
            0 => " ",
            _ => " ",
        }
    }
    /*

    1 |X|X|X|
    2 |X|X|X|
    3 |X|X|X|
       A B C
    */
    fn draw_game(&self, player: i8) {
        let mut draw_str: String = "1 | | | |\n
                                    2 | | | |\n
                                    3 | | | |\n   
                                       A B C"
            .to_string();
        for (i, p) in self.quadrants.iter().enumerate() {
            draw_str.replace_range(i + 3..i + 4, Self::next_player(*p));
        }
        println!("Tic! Tac! Toe!");
        println!("Next player is: {}\n", Self::next_player(player));
        print!("{}", draw_str);
    }
}
