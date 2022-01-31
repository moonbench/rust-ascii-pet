mod character;
#[path="./character/emotion.rs"]
mod emotion;

use crate::character::Character;
use crate::character::emotion::{Emotion, Emotions};

use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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

    let (tx, rx) = mpsc::channel();

    let render_loop = thread::spawn(move || {
        loop {
            match rx.try_recv() {
                Err(something) => {},
                Ok(message) => {
                    pet.set_emotion(message)
                }
            };
            let delay = render_frame(&mut pet);
            thread::sleep(Duration::from_millis(delay));
        }
    });

    let update_loop = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(2500));
            let emotion = Emotions::Happy;
            tx.send(emotion).unwrap();
            thread::sleep(Duration::from_millis(4000));
            let emotion = Emotions::Loving;
            tx.send(emotion).unwrap();
            thread::sleep(Duration::from_millis(5000));
            let emotion = Emotions::Happy;
            tx.send(emotion).unwrap();
            thread::sleep(Duration::from_millis(2500));
            let emotion = Emotions::Excited;
            tx.send(emotion).unwrap();
            thread::sleep(Duration::from_millis(2500));
            let emotion = Emotions::Happy;
            tx.send(emotion).unwrap();
        }
    });

    render_loop.join().unwrap();
    update_loop.join().unwrap();
}
