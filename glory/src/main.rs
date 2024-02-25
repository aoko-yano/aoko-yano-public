pub mod piston;
pub mod data;
mod update;

use piston_window::*;

use update::create_data;
use piston::{KeysState, create_window};

fn main() {
    let mut key_state = KeysState::new();
    let mut window: PistonWindow = create_window();

    let mut data = create_data(10, 10);
    update::put_people_to_data(&mut data,&Position{ x: 0, y: 0 },1);
    while let Some(e) = window.next() {
        match e {
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&e, |c, g, _| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    data.draw(c, g);
                });
            }
            Event::Loop(Loop::Update(_)) => {
                update::update_data(&mut data, &key_state);
            }
            Event::Input(i, _) => {
                if let Input::Button(key) = i {
                    key_state.set(&key);
                }
            }
            _ => {}
        }
    }
}