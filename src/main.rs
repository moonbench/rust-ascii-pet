mod character;

use crate::character::Character;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crossterm::{
    execute,
    queue,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor, Print},
    event,
    terminal,
    cursor,
};

fn draw(text: &str, position: (u16, u16)) {
    queue!(
        std::io::stdout(),
        cursor::MoveTo(position.0, position.1),
        Print(text),
    ).unwrap();
}

fn set_environment_colors() {
    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Rgb{r: 12, g: 12, b: 40}),
    ).unwrap();
}

fn render_frame(pet: &mut Character) -> u64 {
    set_environment_colors();

    // Print this frame
    for row in 5..=6 {
        draw(&format!("║{: ^40}║", ""), (0, row));
    }
    let delay = pet.next_tick();
    execute!(
        std::io::stdout(),
        cursor::MoveTo(0, 11),
        SetForegroundColor(Color::Blue),
        Print(format!("{:^42}", format!("It looks {}...", pet.emotion.name.to_string().to_lowercase()))),
        ResetColor
    ).unwrap();

    // print!("{:#?}\n", pet.vitals);
    delay
}

fn render_environment(name: &str) {
    set_environment_colors();

    // Draw the enclosure
    draw(&format!("╔{:═^40}╗", format!(" {} ", name)), (0,0));
    for row in 1..=6 {
        draw(&format!("║{: ^40}║", ""), (0, row));
    }
    draw(&format!("║{:░^40}║", ""), (0,7));
    draw(&format!("║{:▒^40}║", ""), (0,8));
    draw(&format!("║{:▓^40}║", ""), (0,9));
    draw(&format!("╚{:═^40}╝", ""), (0,10));
    draw(&format!("{: ^42}", ""), (0,11));
    draw(&format!("{:─^42}", ""), (0,12));
    draw(&format!("{:^42}", ""), (0,13));
}

fn draw_menu() {
    execute!(
        std::io::stdout(),
        cursor::MoveTo(1, 13),
        Print("q: Exit"),
    ).unwrap();
}

fn setup_terminal() {
    terminal::enable_raw_mode().unwrap();
    execute!(
        std::io::stdout(),
        terminal::EnterAlternateScreen,
        terminal::SetSize(42, 14),
        terminal::SetTitle("Buddy"),
        cursor::Hide
    ).unwrap();
}

fn restore_terminal(original_size: (u16, u16)) {
    terminal::disable_raw_mode().unwrap();
    execute!(
        std::io::stdout(),
        cursor::Show,
        ResetColor,
        terminal::SetSize(original_size.0, original_size.1),
        terminal::LeaveAlternateScreen
    ).unwrap();
}

fn main() {
    let (update_tx, update_rx) = mpsc::channel();
    let (delay_tx, delay_rx) = mpsc::channel();
    let original_size = terminal::size().unwrap();
    let mut pet = Character::default();

    setup_terminal();
    render_environment(&pet.name);
    draw_menu();

    // Animation thread
    thread::spawn(move || {
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

    // Time-passage monitor thread
    thread::spawn(move || {
        loop {
            update_tx.send("state_change").unwrap();
            let delay = match delay_rx.recv() {
                Err(_) => {2000},
                Ok(ms) => ms
            };
            thread::sleep(Duration::from_millis(delay));
        }
    });

    // Input query thread
    let input_loop = thread::spawn(move || {
        loop {
            if event::poll(Duration::from_secs(1)).unwrap() {
                match event::read().unwrap() {
                    event::Event::Key(key_press) => match key_press.code {
                        event::KeyCode::Char('q') => break,
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    });

    input_loop.join().unwrap();
    restore_terminal(original_size);
}
