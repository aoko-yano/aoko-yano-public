pub mod planet;

use crate::planet::create_planet;

fn main() {
    let mut planet = create_planet();
    planet.put_first_people();
    let mut turn = 0;
    println!("{:?}", &planet);
    while planet.is_still_survives() {
        println!("================ Turn: {} ================", turn);
        turn += 1;
        planet.tick();
        println!("{:?}", &planet);
        if planet.is_finished() {
            break;
        }
    }
}
