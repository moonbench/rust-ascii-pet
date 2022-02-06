mod character;

use crate::character::Character;
use crate::character::emotion::Emotions;

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

    for row in 1..=7 {
        draw(&format!("║{: ^40}║", ""), (0, row));
    }
    let delay = pet.next_tick();

    render_statusbar(pet);

    delay
}

fn render_statusbar(pet: &Character) {
    let energy = format!("╠ 🗲 {}", pet.vitals.energy);
    let joy =    format!("╠ ♥ {}", pet.vitals.happiness);
    execute!(
        std::io::stdout(),
        SetForegroundColor(Color::Blue),
        cursor::MoveTo(2, 1),
        Print(format!("{:>38} ╣", pet.emotion.name.to_string().to_lowercase())),
        cursor::MoveTo(0,1),
        Print(format!("{:<6}", energy)),
        cursor::MoveTo(0,2),
        Print(format!("{:<6}", joy)),
    ).unwrap();
}

fn render_environment(name: &str) {
    set_environment_colors();

    // Draw the enclosure
    draw(&format!("╔{:═^40}╗", format!(" {} ", name)), (0,0));
    draw(&format!("║{:░^40}║", ""), (0,8));
    draw(&format!("║{:▒^40}║", ""), (0,9));
    draw(&format!("║{:▓^40}║", ""), (0,10));
    draw(&format!("╚{:═^40}╝", ""), (0,11));
    draw(&format!("{:^42}", ""), (0,12));
    draw(&format!("{:^42}", ""), (0,13));
}

fn draw_menu() {
    execute!(
        std::io::stdout(),
        cursor::MoveTo(1, 13),
        Print("q: Exit   f: Feed"),
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
    execute!(
        std::io::stdout(),
        cursor::Show,
        ResetColor,
        terminal::Clear(terminal::ClearType::All),
        terminal::LeaveAlternateScreen,
        terminal::SetSize(original_size.0, original_size.1)
    ).unwrap();
    terminal::disable_raw_mode().unwrap();
}

fn main() {
    let (update_tx, update_rx) = mpsc::channel();
    let (delay_tx, delay_rx) = mpsc::channel();
    let (input_tx, input_rx) = mpsc::channel();
    let original_size = terminal::size().unwrap();
    let mut pet = Character::default();

    setup_terminal();
    render_environment(&pet.name);
    draw_menu();

    // Animation thread
    let game_loop = thread::spawn(move || {
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
            match input_rx.recv_timeout(Duration::from_millis(delay)) {
                Err(_) => {},
                Ok(message) => {
                    match message {
                        "feed" => pet.set_state(Emotions::Eating),
                        "end" => break,
                        _ => {}
                    }
                    delay_tx.send(pet.animation.duration).unwrap();
                }
            };
        }
    });

    // Time-passage monitor thread
    thread::spawn(move || {
        let mut delay = 5000;
        loop {
            match delay_rx.recv_timeout(Duration::from_millis(delay)) {
                Err(_) => update_tx.send("state_change").unwrap(),
                Ok(ms) => delay = ms,
            }
        }
    });

    // Input query thread
    thread::spawn(move || {
        loop {
            match event::read().unwrap() {
                event::Event::Key(key_press) => match key_press.code {
                    event::KeyCode::Char('q') | event::KeyCode::Esc => input_tx.send("end").unwrap(),
                    event::KeyCode::Char('f') => input_tx.send("feed").unwrap(),
                    _ => {}
                },
                _ => {}
            }
        }
    });

    game_loop.join().unwrap();
    restore_terminal(original_size);
}
