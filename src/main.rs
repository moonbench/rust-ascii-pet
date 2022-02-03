mod character;

use crate::character::Character;

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
    print!("║{:░^30}║\n", "");
    print!("║{:▒^30}║\n", "");
    print!("║{:▓^30}║\n", "");
    print!("╚{:═^30}╝\n", "");
    print!("Feeling: {}\n", pet.emotion.name_string());

    // Flush the output buffer all at once
    std::io::stdout().flush().unwrap();

    delay
}

fn main() {
    let mut pet = Character::default();

    let (update_tx, update_rx) = mpsc::channel();
    let (delay_tx, delay_rx) = mpsc::channel();

    let render_loop = thread::spawn(move || {
        loop {
            match update_rx.try_recv() {
                Err(_) => {},
                Ok(message) => {
                    match message {
                        "state_change" => pet.state_change(),
                        _ => {}
                    }
                    delay_tx.send(pet.animation.duration).unwrap();
                }
            };
            let delay = render_frame(&mut pet);
            thread::sleep(Duration::from_millis(delay));
        }
    });

    let update_loop = thread::spawn(move || {
        loop {
            update_tx.send("state_change").unwrap();
            let delay = match delay_rx.recv() {
                Err(_) => {2000},
                Ok(ms) => ms
            };

            thread::sleep(Duration::from_millis(delay));
        }
    });

    render_loop.join().unwrap();
    update_loop.join().unwrap();
}
