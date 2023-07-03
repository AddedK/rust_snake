use piston_window::Key;
use rand::distributions::{Distribution, Uniform};
use core::num;
use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position {
    row: i32,
    column: i32,
}

impl Position {
    pub fn new(row: i32, column: i32) -> Position {
        Position {row, column}
    }

    pub fn get_row(&self) -> i32 {
        self.row
    }

    pub fn get_column(&self) -> i32 {
        self.column
    }

}

#[derive(PartialEq, Debug)]
pub struct Game {
    num_rows: usize,
    num_cols: usize,
    board: Vec<Vec<u8>>,
    snake_body: VecDeque<Position>,
    current_snake_direction: Direction,
    next_snake_position: Option<Direction>,
    food_position: Position,
}

impl Default for Game {
    fn default() -> Self {
        let num_rows = 10;
        let num_cols = 10;
        let board = vec![vec![0; num_cols]; num_rows];
        let mut snake_body = VecDeque::new();
        snake_body.push_front(Position::new(1,1));
        snake_body.push_front(Position::new(2,1));
        let current_snake_direction = Direction::Right;
        let next_snake_position = None;
        let food_position = Position::new(2,2);
        Game {
            num_rows,
            num_cols,
            board,
            snake_body,
            current_snake_direction,
            next_snake_position,
            food_position,
        }
    }
}

impl Game {
    pub fn new(
        num_rows: usize,
        num_cols: usize,
        mut snake_body: VecDeque<Position>,
        current_snake_direction: Direction,
        food_position: Position,
    ) -> Game {
        if num_rows == 0 {
            println!("num_rows is zero. Defaulting");
            return Game::default();
        }
        if num_cols == 0 {
            println!("num_cols is zero. Defaulting");
            return Game::default();
        }

        let mut board = vec![vec![0; num_cols]; num_rows];
        for snake_position in &snake_body {
            if snake_position.row < 0
                || snake_position.row as usize >= num_rows
                || snake_position.column < 0
                || snake_position.column as usize >= num_cols
            {
                println!("Snake is out of bounds. Defaulting");
                return Game::default();
            }
            board[snake_position.row as usize][snake_position.column as usize] = 1;
        }
        if snake_body.is_empty() {
            if food_position == Position::new(0, 0) {
                snake_body.push_front(Position::new(1, 1));
            } else {
                snake_body.push_front(Position::new(0, 0));
            }
        }
        if food_position.row < 0
            || food_position.row as usize >= num_rows
            || food_position.column < 0
            || food_position.column as usize >= num_cols
        {
            println!("Food is out of bounds. Defaulting");
            return Game::default();
        }

        let next_snake_position = None;
        Game {
            num_rows,
            num_cols,
            board,
            snake_body,
            current_snake_direction,
            next_snake_position,
            food_position,
        }
    }

    pub fn get_num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn get_num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn get_snake_positions(&self) -> &VecDeque<Position> {
        &self.snake_body
    }

    pub fn get_food_position(&self) -> &Position {
        &self.food_position
    }

    pub fn handle_key(&mut self, key: Key) {
        match key {
            Key::Left => self.next_snake_position = Some(Direction::Left),
            Key::Up => self.next_snake_position = Some(Direction::Up),
            Key::Right => self.next_snake_position = Some(Direction::Right),
            Key::Down => self.next_snake_position = Some(Direction::Down),
            _ => (),
        }
    }

    pub fn update_direction(&mut self) {
        match self.next_snake_position {
            Some(Direction::Left) => {
                if self.current_snake_direction != Direction::Right {
                    self.current_snake_direction = Direction::Left
                }
            }
            Some(Direction::Up) => {
                if self.current_snake_direction != Direction::Down {
                    self.current_snake_direction = Direction::Up
                }
            }
            Some(Direction::Right) => {
                if self.current_snake_direction != Direction::Left {
                    self.current_snake_direction = Direction::Right
                }
            }
            Some(Direction::Down) => {
                if self.current_snake_direction != Direction::Up {
                    self.current_snake_direction = Direction::Down
                }
            }
            _ => (),
        }
        self.next_snake_position = None;
    }

    pub fn check_if_hit_wall(&mut self) -> Result<(), &'static str> {
        let head = self.snake_body.front().unwrap();
        if head.row >= self.num_rows as i32
            || head.row < 0
        {
            return Err("Snake hit top or bottom wall");
        } else if head.column >= self.num_cols as i32
            || head.column < 0
        {
            return Err("Snake head hit left or right wall");
        }
        Ok(())
    }

    pub fn check_if_hit_snake(&mut self) -> Result<(), &'static str> {
        let head = self.snake_body.front().unwrap();
        if self.board[head.row as usize]
            [head.column as usize]
            == 1
        {
            return Err("Snake hit itself");
        }

        Ok(())
    }

    pub fn snake_found_food(&mut self) -> bool {
        self.snake_body.front().unwrap() == &self.food_position
    }

    pub fn spawn_new_food(&mut self) -> Result<(), &'static str> {
        let mut rng = rand::thread_rng();

        let mut valid_new_position = Vec::new();

        for row in 0..self.board.len() {
            for col in 0..self.board[0].len() {
                if self.board[row][col] == 0 {
                    valid_new_position.push(Position::new(row as i32, col as i32));
                }
            }
        }
        if valid_new_position.is_empty() {
            return Err("No room to spawn food");
        }

        let rand_die = Uniform::from(0..valid_new_position.len());
        let rand_index = rand_die.sample(&mut rng);
        self.food_position = valid_new_position[rand_index];
        Ok(())
    }

    pub fn move_snake(&mut self) -> Result<(), &'static str> {
        let old_head = self.snake_body.front().unwrap();
        match self.current_snake_direction {
            Direction::Left => self.snake_body.push_front(Position::new(old_head.row, old_head.column-1)),
            Direction::Up => self.snake_body.push_front(Position::new(old_head.row-1, old_head.column)),
            Direction::Right => self.snake_body.push_front(Position::new(old_head.row, old_head.column+1)),
            Direction::Down => self.snake_body.push_front(Position::new(old_head.row+1, old_head.column)),
        }

        self.check_if_hit_wall()?;
        self.check_if_hit_snake()?;

        let new_head = self.snake_body.front().unwrap();
        self.board[new_head.row as usize][new_head.column as usize] = 1;

        if self.snake_found_food() {
            self.spawn_new_food()?;
        } else {
            let tail = self.snake_body.pop_back().unwrap();
            self.board[tail.row as usize][tail.column as usize] = 0;
        }

        Ok(())
    }

    pub fn update_game(&mut self) -> Result<(), &'static str> {
        self.update_direction();
        self.move_snake()?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn create_basic_game() -> Game {
        let num_rows = 10;
        let num_cols = 10;
        let mut snake_body = VecDeque::new();
        snake_body.push_front(Position::new(4, 4));
        let current_snake_direction = Direction::Right;
        let food_position = Position::new(2, 2);

        Game::new(
            num_rows,
            num_cols,
            snake_body,
            current_snake_direction,
            food_position,
        )
    }

    fn create_almost_full_game(num_cols: usize) -> Game {
        let num_rows = 1;
        let mut snake_body = VecDeque::new();
        for i in 0..num_cols-1 {
            snake_body.push_front(Position::new(0, i as i32));
        }
        let current_snake_direction = Direction::Right;
        let food_position = Position::new(0, num_cols as i32 - 1);

        Game::new(
            num_rows,
            num_cols,
            snake_body,
            current_snake_direction,
            food_position,
        )
    }

    #[test]
    fn new_game_empty_snake_body_food_collision() {
        let num_rows = 10;
        let num_cols = 10;
        let snake_body = VecDeque::new();
        let current_snake_direction = Direction::Right;
        let food_position = Position::new(0, 0);

        let bad_game = Game::new(
            num_rows,
            num_cols,
            snake_body,
            current_snake_direction,
            food_position,
        );
        assert_eq!(bad_game.snake_body.front().unwrap(), &Position::new(1, 1));
    }

    #[test]
    fn new_game_snake_body_out_of_bounds() {
        let default_game = Game::default();

        let num_rows = 5;
        let num_cols = 5;
        let mut snake_body = VecDeque::new();
        snake_body.push_front(Position::new(4, 4));
        snake_body.push_front(Position::new(5, 5));
        let current_snake_direction = Direction::Right;
        let food_position = Position::new(0, 0);

        let bad_game = Game::new(
            num_rows,
            num_cols,
            snake_body,
            current_snake_direction,
            food_position,
        );

        assert_eq!(bad_game, default_game);
    }

    #[test]
    fn new_game_snake_body_within_bounds() {
        let default_game = Game::default();

        let num_rows = 5;
        let num_cols = 5;
        let mut snake_body = VecDeque::new();
        snake_body.push_front(Position::new(0, 0));
        snake_body.push_front(Position::new(1, 1));
        let current_snake_direction = Direction::Right;
        let food_position = Position::new(4, 4);

        let bad_game = Game::new(
            num_rows,
            num_cols,
            snake_body,
            current_snake_direction,
            food_position,
        );

        assert_ne!(bad_game, default_game);
    }

    #[test]
    fn new_game_food_out_of_bounds() {
        let default_game = Game::default();

        let num_rows = 5;
        let num_cols = 5;
        let mut snake_body = VecDeque::new();
        snake_body.push_front(Position::new(1, 1));
        let current_snake_direction = Direction::Right;
        let food_position = Position::new(5, 5);

        let bad_game = Game::new(
            num_rows,
            num_cols,
            snake_body,
            current_snake_direction,
            food_position,
        );

        assert_eq!(bad_game, default_game);
    }

    #[test]
    fn handle_key_next_direction_default() {
        let mut game = create_basic_game();
        assert_eq!(game.next_snake_position, None);
    }

    #[test]
    fn update_direction_not_opposite_movement() {
        let mut game = create_basic_game();

        game.handle_key(Key::Down);
        assert_eq!(game.next_snake_position, Some(Direction::Down));
        game.update_direction();
        assert_eq!(game.current_snake_direction, Direction::Down);
        assert_eq!(game.next_snake_position, None);

        game.handle_key(Key::Left);
        assert_eq!(game.next_snake_position, Some(Direction::Left));
        game.update_direction();
        assert_eq!(game.current_snake_direction, Direction::Left);

        game.handle_key(Key::Up);
        assert_eq!(game.next_snake_position, Some(Direction::Up));
        game.update_direction();
        assert_eq!(game.current_snake_direction, Direction::Up);

        game.handle_key(Key::Right);
        assert_eq!(game.next_snake_position, Some(Direction::Right));
        game.update_direction();
        assert_eq!(game.current_snake_direction, Direction::Right);
    }

    #[test]
    fn update_direction_opposite_movement() {
        let mut game = create_basic_game();

        game.handle_key(Key::Left);
        game.update_direction();
        assert_eq!(game.current_snake_direction, Direction::Right);

        game.handle_key(Key::Up);
        game.update_direction();
        game.handle_key(Key::Down);
        game.update_direction();
        assert_eq!(game.current_snake_direction, Direction::Up);

        game.handle_key(Key::Right);
        game.update_direction();
        game.handle_key(Key::Left);
        game.update_direction();
        assert_eq!(game.current_snake_direction, Direction::Right);

        game.handle_key(Key::Down);
        game.update_direction();
        game.handle_key(Key::Up);
        game.update_direction();
        assert_eq!(game.current_snake_direction, Direction::Down);

        game.handle_key(Key::Left);
        game.update_direction();
        game.handle_key(Key::Right);
        game.update_direction();
        assert_eq!(game.current_snake_direction, Direction::Left);
    }

    #[test]
    fn snake_does_not_hit_wall() {
        let mut game = create_basic_game();
        let res = game.check_if_hit_wall();
        assert!(res.is_ok());
    }

    #[test]
    fn snake_hits_wall() {
        let mut game = create_basic_game();
        game.snake_body.pop_front().unwrap();
        game.snake_body.push_front(Position::new(10, 10));
        let res = game.check_if_hit_wall();
        assert!(res.is_err());
    }

    #[test]
    fn snake_does_not_hit_snake() {
        let mut game = create_basic_game();
        game.snake_body.pop_front().unwrap();
        game.snake_body.push_front(Position::new(5, 5));
        let res = game.check_if_hit_snake();
        assert!(res.is_ok());
    }

    #[test]
    fn snake_does_not_find_food() {
        let mut game = create_basic_game();
        let res = game.snake_found_food();
        assert!(!res);
    }
    #[test]
    fn snake_does_find_food() {
        let mut game = create_basic_game();
        game.snake_body.pop_front().unwrap();
        game.snake_body.push_front(Position::new(2, 2));
        let res = game.snake_found_food();
        assert!(res);
    }
    #[test]
    fn spawn_food_fails_with_full_small_board() {
        let mut game = create_almost_full_game(2);
        let res = game.move_snake();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "No room to spawn food");
    }

    #[test]
    fn spawn_food_fails_with_full_big_board() {
        let mut game = create_almost_full_game(10);
        let res = game.move_snake();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "No room to spawn food");
    }
    #[test]
    fn spawn_food_works() {
        let mut game = create_basic_game();
        game.handle_key(Key::Up);
        game.update_direction();

        assert!(game.move_snake().is_ok());
        assert!(game.move_snake().is_ok());

        game.handle_key(Key::Left);
        game.update_direction();

        assert!(game.move_snake().is_ok());
        assert!(game.move_snake().is_ok());
    }

    
}
