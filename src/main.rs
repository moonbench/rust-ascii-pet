mod character;

use crate::character::Character;

use std::thread;
use std::time::Duration;
use std::io::Write;

fn render_frame(pet: &mut Character) -> u64 {
    // Clear the last
    print!("{esc}c", esc = 27 as char);

    // Print this frame
    print!("╔{:═^30}╗\n", format!(" {} ", pet.name));
    let delay = pet.next_tick();
    print!("╚═╦{:═^26}╦═╝\n", "");
    print!("  █{: ^26}█ \n", "");

    // Flush the output buffer all at once
    std::io::stdout().flush().unwrap();

    delay
}

fn main() {
    let mut pet = Character::default();
    pet.pick_emotion();

    loop {
        let delay = render_frame(&mut pet);
        thread::sleep(Duration::from_millis(delay));
    }
}
