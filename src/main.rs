extern crate piston_window;
extern crate rand;

use piston_window::types::Color;
use piston_window::*;

mod draw;
mod game;
mod snake;

use draw::to_coord_u32;
use game::Game;

const BACKGROUND: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (40, 40);

    let mut window: PistonWindow =
        WindowSettings::new("snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACKGROUND, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
