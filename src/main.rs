extern crate piston_window;
extern crate rand;
extern crate find_folder;

use piston_window::*;
use piston_window::types::Color;
use crate::game::Game;

mod player;
mod my_draw;
mod game;
mod opponent;
mod bullet;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const SPAWN_INTERVAL : f64 = 2.0;



fn main() {

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();


    let mut game = Game::new(640, 480);

    while let Some(event) = window.next() {
       if let Some(Button::Keyboard(key)) = event.press_args() {
           game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g| {
            let transform = c.transform.trans(10.0, 50.0);

             clear(BACK_COLOR, g);
            game.draw_board(&c, g);
            text::Text::new(25).draw(&format!("{}",game.get_count()), &mut glyphs, &c.draw_state,transform, g)
        });

        event.update(|arg| {
            game.waiting_time += arg.dt;

            if game.waiting_time > SPAWN_INTERVAL {
                game.spawn_opponent();
                game.waiting_time = 0.0;
            }

            game.update(arg.dt);
        });
    }
}
