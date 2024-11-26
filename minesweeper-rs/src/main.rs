mod MineSweeper;
mod utils;

use MineSweeper::STATUS;
use std::io;

fn main() {
    let mut game = MineSweeper::MineSweeper::new(15, 15, 10);

    while game.get_status() == STATUS::PLAYING {
        println!("{}", game);
        println!("Open Cell -> 1 [x] [y]");
        println!("Flag Cell -> 2 [x] [y]");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() == 3 {
            let command = parts[0];
            let x: usize = parts[1].parse().expect("Invalid x coordinate.");
            let y: usize = parts[2].parse().expect("Invalid y coordinate.");

            match command {
                "1" => {
                    game.open_cell(x, y);
                }
                "2" => {
                    game.toggle_flag(x, y);
                }
                _ => {
                    println!("Invalid command.");
                }
            }
        } else {
            println!("Invalid input format.");
        }
    }

    if game.get_status() == STATUS::LOSE {
        println!("Game over! You lose!");
    } else if game.get_status() == STATUS::WIN {
        println!("Congratulations! You win!");
    }
}
