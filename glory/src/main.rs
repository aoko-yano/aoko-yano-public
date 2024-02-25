pub mod piston;
pub mod data;

use piston_window::*;

use data::create_data;
use piston::{ArrowKeysState, create_window};

fn main() {
    let mut arrow_keys = ArrowKeysState::new();
    let mut window: PistonWindow = create_window();

    let mut data = create_data(10, 10);
    data.put_first_people(0,0);
    while let Some(e) = window.next() {
        match e {
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&e, |c, g, _| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    data.draw(c, g);
                });
            }
            Event::Loop(Loop::Update(_)) => {
                data.update();
            }
            Event::Input(i, _) => {
                if let Input::Button(key) = i {
                    arrow_keys.set(&key);
                }
            }
            _ => {}
        }
    }
}