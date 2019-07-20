use ndarray::{Array2, Axis};
use std::fmt::{Display, Formatter, Error, Write};
use ndarray_parallel::prelude::ParMap;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

const BOARD_SIZE: (usize, usize) = (7, 7);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    UP, DOWN, LEFT, RIGHT
}

#[readonly::make]
pub struct SnakeGame {
    /// board consist of number timer, value decrease every tick clamped to 0 so it eventually become 0
    ///
    /// for each cell in array -> 0 = empty, >0 = snake part exist there
    pub board: Array2<u32>,
    pub food_pos: (usize, usize),  // (y, x) due to array index notation
    pub snake_pos: (usize, usize), // (y, x) due to array index notation, head of snake
    pub snake_dir: Direction, // snake direction of motion
    pub snake_lvl: u32,  // determine how long snake part persist (increase when eat food)
    rng: ThreadRng,
}

impl SnakeGame {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut game = SnakeGame {
            board: Array2::zeros(BOARD_SIZE),
            food_pos: (rng.gen_range(0, BOARD_SIZE.0), rng.gen_range(0, BOARD_SIZE.1)),
            snake_pos: (BOARD_SIZE.0/2, BOARD_SIZE.1/2),
            snake_dir: Direction::RIGHT,
            snake_lvl: 3,
            rng
        };
        game.tick(None);
        game
    }

    fn random_pos(&mut self) -> (usize, usize) {
        let (size_0, size_1) = self.board_size();
        (self.rng.gen_range(0, size_0), self.rng.gen_range(0, size_1))
    }

    fn random_food_pos(&mut self) -> (usize, usize) {
        loop {
            let pos = self.random_pos();
            if self.board[pos] == 0 {break pos}
        }
    }

    fn board_size(&self) -> (usize, usize) {
        (self.board.len_of(Axis(0)), self.board.len_of(Axis(1)))
    }

    // set snake dirction of motion
    fn set_direction(&mut self, dir: Direction) {
        match dir {
            Direction::UP => if self.snake_dir != Direction::DOWN { self.snake_dir = dir },
            Direction::DOWN => if self.snake_dir != Direction::UP { self.snake_dir = dir },
            Direction::LEFT => if self.snake_dir != Direction::RIGHT { self.snake_dir = dir },
            Direction::RIGHT => if self.snake_dir != Direction::LEFT { self.snake_dir = dir },
        }
    }

    /// move snake forward in direction self.snake_dir
    /// * does perform boundary wrapping
    /// * does not perform collision checking (refer to: check_snake_collision)
    fn move_snake_head(&mut self) {
        let board_size = self.board_size();
        match self.snake_dir {
            Direction::UP =>
                self.snake_pos.0 = self.snake_pos.0.checked_sub(1).unwrap_or(board_size.0 - 1),
            Direction::DOWN =>
                self.snake_pos.0 = Some(self.snake_pos.0 + 1).filter(|&x| x < board_size.0).unwrap_or(0),
            Direction::LEFT =>
                self.snake_pos.1 = self.snake_pos.1.checked_sub(1).unwrap_or(board_size.1 - 1),
            Direction::RIGHT =>
                self.snake_pos.1 = Some(self.snake_pos.1 + 1).filter(|&x| x < board_size.1).unwrap_or(0),
        }
    }

    /// check if sometime is collided with snake head and called appropriate method,
    /// should be call after moving snake head and before setting value to board
    fn check_snake_collision(&mut self) {
        if self.board[self.snake_pos] != 0 {
            // collide with self
            self.snake_collided();
        }
        if self.snake_pos == self.food_pos {
            // collide with food
            self.food_collided();
        }
    }

    /// when snake collide with food, do this
    fn food_collided(&mut self) {
        self.snake_lvl += 1;
        self.food_pos = self.random_food_pos();
    }

    /// when snake hit itself, do this
    fn snake_collided(&mut self) {
        // TODO: implement
        unimplemented!();
    }

    /// Main Game Logic
    ///
    /// snake_dir_input: Some(dir) = change direction to dir, None = no direction change
    pub fn tick(&mut self, snake_dir_input: Option<Direction>) {

        // get input and may be change snake direction
        if let Some(input) = snake_dir_input {
            self.set_direction(input);
        }

        // decrease value (timer) on grid
        self.board.par_map_inplace(|x| *x = x.saturating_sub(1));

        // move snake head
        self.move_snake_head();

        // check snake collision
        self.check_snake_collision();

        // add value to snake head's cell
        self.board[self.snake_pos] = self.snake_lvl;
    }

}

impl Display for SnakeGame {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {

        writeln!(f, "{}", "-".repeat(self.board.len_of(Axis(0))))?;

        for (ax0, row) in self.board.axis_iter(Axis(0)).enumerate() {
            for (ax1, cell) in row.iter().enumerate() {
                if *cell > 0 {
                    if (ax0, ax1) == self.snake_pos {f.write_char('@')?}
                    else {f.write_char('#')?}
                }
                else if (ax0, ax1) == self.food_pos {
                    f.write_char('F')?
                }
                else {
                    f.write_char(' ')?
                }
            }
            f.write_char('\n')?
        }

        writeln!(f, "{}", "-".repeat(self.board.len_of(Axis(0))))?;

        Ok(())
    }
}

impl Default for SnakeGame {
    fn default() -> Self {
        Self::new()
    }
}