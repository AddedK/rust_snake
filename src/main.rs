use piston_window::types::Color;
use piston_window::Button::Keyboard;
use piston_window::*;
use rand::Rng;
use std::process;
use std::thread;
use std::time::Duration;

use rand::distributions::{Distribution, Uniform};
use std::collections::VecDeque;

mod game;
use game::Game;

fn my_render(
    event: Event,
    window: &mut PistonWindow,
    rng: &mut rand::rngs::ThreadRng,
    game: &Game,
) {
    let rn1: types::ColorComponent = rng.gen();
    let rn2: types::ColorComponent = rng.gen();
    let rn3: types::ColorComponent = rng.gen();
    let rn4: types::ColorComponent = rng.gen();

    let random_color: Color = [rn1, rn2, rn3, rn4];

    let draw_width_of_one_square = window.size().width / game.get_num_rows() as f64;
    let draw_height_of_one_square = window.size().height / game.get_num_cols() as f64;
    window.draw_2d(&event, |c, g, _| {
        clear([0.5, 0.5, 0.5, 1.0], g);
        for position in game.get_snake_positions() {
            rectangle(
                random_color, // red
                [
                    position.0 as f64 * draw_width_of_one_square,
                    position.1 as f64 * draw_height_of_one_square,
                    draw_width_of_one_square,
                    draw_height_of_one_square,
                ], // rectangle
                c.transform,
                g,
            );
        }
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
    snake_body.push_front((0, 0));
    snake_body.push_front((1, 0));
    snake_body.push_front((2, 2));
    let snake_direction = game::Direction::Right;
    let food_position = (2, 2);

    let mut game = Game::new(
        num_rows,
        num_cols,
        snake_body,
        snake_direction,
        food_position,
    );

    let mut rng = rand::thread_rng();
    let mut window: PistonWindow = WindowSettings::new("Hello World!", [512; 2])
        .build()
        .unwrap_or_else(|err| {
            eprintln!("Failed to build window.");
            eprintln!("{err}");
            process::exit(1);
        });
    // let mut start = Instant::now();
    let mut count = 0;
    while let Some(event) = window.next() {
        // Potentially replace this with if let for close case and button case
        match event {
            Event::Input(Input::Close(_close_args), _) => break, // close
            Event::Input(Input::Button(_button_args), _) => handle_buttons(_button_args, &mut game), // update snake direction
            _ => (),
        }

        my_render(event, &mut window, &mut rng, &game);
        println!("Done rendering {count}");
        count += 1;
        thread::sleep(Duration::from_millis(100));
    }
}
