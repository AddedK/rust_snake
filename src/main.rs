use piston_window::Button::Keyboard;
use piston_window::*;
use std::process;
use std::thread;
use std::time::Instant;

use std::collections::VecDeque;

mod game;
use game::Game;

fn render_game(event: Event, window: &mut PistonWindow, game: &Game) {
    let snake_color = [0.2, 0.6, 0.3, 1.0];
    let food_color = [0.7, 0.3, 0.2, 1.0];

    let draw_width_of_one_square = window.size().width / game.get_num_rows() as f64;
    let draw_height_of_one_square = window.size().height / game.get_num_cols() as f64;

    window.draw_2d(&event, |c, g, _| {
        clear([0.5, 0.5, 0.5, 1.0], g);
        for position in game.get_snake_positions() {
            rectangle(
                snake_color,
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
            food_color,
            [
                food_position.get_column() as f64 * draw_width_of_one_square,
                food_position.get_row() as f64 * draw_height_of_one_square,
                draw_width_of_one_square,
                draw_height_of_one_square,
            ],
            c.transform,
            g,
        );
    });
}

fn handle_buttons(button: ButtonArgs, game: &mut Game) {
    let small_button = button.button;
    if let Keyboard(key) = small_button {
        game.handle_key(key);
    }
}

fn main() {
    let num_rows = 10;
    let num_cols = 10;
    let mut snake_body = VecDeque::new();
    snake_body.push_front(game::Position::new(0, 0));
    snake_body.push_front(game::Position::new(1, 0));
    snake_body.push_front(game::Position::new(2, 0));
    let snake_direction = game::Direction::Right;
    let food_position = game::Position::new(5, 5);

    let mut game = Game::new(
        num_rows,
        num_cols,
        snake_body,
        snake_direction,
        food_position,
    );

    let mut window: PistonWindow = WindowSettings::new("Hello World!", [512; 2])
        .build()
        .unwrap_or_else(|err| {
            eprintln!("Failed to build window.");
            eprintln!("{err}");
            process::exit(1);
        });
    let mut count = 0;
    let mut start_time = Instant::now();
    while let Some(event) = window.next() {
        let duration = start_time.elapsed();

        match event {
            Event::Input(Input::Close(_close_args), _) => break,
            Event::Input(Input::Button(_button_args), _) => handle_buttons(_button_args, &mut game),
            _ => (),
        }
        if duration.as_millis() > 250 {
            if let Err(err) = game.update_game() {
                eprintln!("Update game failed: {err}");
                break;
            }
            start_time = Instant::now();
        }

        render_game(event, &mut window, &game);
        if count % 100 == 0 {
            println!("Done rendering {count}");
        }
        count += 1;
    }
}
