use piston_window::Button::Keyboard;
use piston_window::*;
use std::process;
use std::time::Instant;

use std::collections::VecDeque;

mod game;
use game::Game;

static SNAKE_COLOR: [f32; 4] = [0.2, 0.6, 0.3, 1.0];
static FOOD_COLOR: [f32; 4] = [0.7, 0.3, 0.2, 1.0];
static BACKGROUND_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
static GAME_OVER_OVERLAY_COLOR: [f32; 4] = [0.8, 0.1, 0.1, 0.2];

static GAME_UPDATE_SPEED: u128 = 250;
static GAME_OVER_SLEEP_MS: u128 = 500;

fn render_game(event: Event, window: &mut PistonWindow, game: &Game, game_over: bool) {
    let window_width = window.size().width;
    let window_height: f64 = window.size().height;
    let draw_width_of_one_square = window.size().width / game.get_num_rows() as f64;
    let draw_height_of_one_square = window.size().height / game.get_num_cols() as f64;

    window.draw_2d(&event, |c, g, _| {
        clear(BACKGROUND_COLOR, g);
        for position in game.get_snake_positions() {
            rectangle(
                SNAKE_COLOR,
                [
                    position.get_column() as f64 * draw_width_of_one_square,
                    position.get_row() as f64 * draw_height_of_one_square,
                    draw_width_of_one_square,
                    draw_height_of_one_square,
                ],
                c.transform,
                g,
            );
        }
        let food_position = game.get_food_position();
        rectangle(
            FOOD_COLOR,
            [
                food_position.get_column() as f64 * draw_width_of_one_square,
                food_position.get_row() as f64 * draw_height_of_one_square,
                draw_width_of_one_square,
                draw_height_of_one_square,
            ],
            c.transform,
            g,
        );
        if game_over {
            rectangle(
                GAME_OVER_OVERLAY_COLOR,
                [0.0, 0.0, window_width, window_height],
                c.transform,
                g,
            );
        }
    });
}

fn handle_buttons(button: ButtonArgs, game: &mut Game) {
    let inner_button = button.button;
    if let Keyboard(key) = inner_button {
        game.handle_key(key);
    }
}

fn create_game() -> Game {
    let num_rows = 10;
    let num_cols = 10;
    let mut snake_body = VecDeque::new();
    snake_body.push_front(game::Position::new(0, 0));
    snake_body.push_front(game::Position::new(0, 1));
    snake_body.push_front(game::Position::new(0, 2));
    let snake_direction = game::Direction::Right;
    let food_position = game::Position::new(5, 5);

    Game::new(
        num_rows,
        num_cols,
        snake_body,
        snake_direction,
        food_position,
    )
}

fn main() {
    let mut game = create_game();

    let mut window: PistonWindow = WindowSettings::new("Hello World!", [512; 2])
        .build()
        .unwrap_or_else(|err| {
            eprintln!("Failed to build window.");
            eprintln!("{err}");
            process::exit(1);
        });
    let mut start_time = Instant::now();
    let mut game_over_time = Instant::now();

    let mut game_over = false;
    while let Some(event) = window.next() {
        if game_over {
            if game_over_time.elapsed().as_millis() > GAME_OVER_SLEEP_MS {
                game = create_game();
                game_over = false;
            }
        } else {
            let duration = start_time.elapsed();

            match event {
                Event::Input(Input::Close(_close_args), _) => break,
                Event::Input(Input::Button(_button_args), _) => {
                    handle_buttons(_button_args, &mut game)
                }
                _ => (),
            }
            if duration.as_millis() > GAME_UPDATE_SPEED {
                if let Err(err) = game.update_game() {
                    eprintln!("Game over cause: {err}");
                    game_over = true;
                    game_over_time = Instant::now();
                }
                start_time = Instant::now();
            }
        }

        render_game(event, &mut window, &game, game_over);
    }
}
