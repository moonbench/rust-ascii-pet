mod character;

use crate::character::Character;

use std::thread;
use std::time::Duration;
use std::io::Write;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let mut pet = Character::default();
    pet.pick_emotion();

    loop {
        print!("{esc}c", esc = 27 as char);
        print!("╔{:═^22}╗\n\n\n", format!(" {} ", pet.name));
        let delay = pet.next_tick();
        print!("\n\n╩╦{:═^20}╦╩\n", "");
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay));
    }
}
