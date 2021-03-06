use console::Term;

use rust_snake::{Direction, SnakeGame};
use text_io::{read, try_read, try_scan};

fn main() {
    let terminal = Term::stdout();

    let mut game = SnakeGame::new(7, 7, 3);

    println!("{}", game);

    loop {
        let str_in: String = read!();

        for i in str_in.chars() {
            let dir = match i {
                'w' => Some(Direction::UP),
                'a' => Some(Direction::LEFT),
                's' => Some(Direction::DOWN),
                'd' => Some(Direction::RIGHT),
                _ => None,
            };
            game.tick(dir);
        }

        terminal.clear_screen().unwrap();
        println!("{}", game);
    }
}
