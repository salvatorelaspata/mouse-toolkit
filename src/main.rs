use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use std::io::{stdout, Write};

mod mouse_move;

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut mouse_move = mouse_move::MouseMove::new(0, 0);

    loop {
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('q') => break,
                _ => {
                    println!("You pressed: {:?}", event.code);
                    stdout().flush().unwrap();
                    match event.code {
                        KeyCode::Up | KeyCode::Char('w') => mouse_move.move_up(),
                        KeyCode::Down | KeyCode::Char('s') => mouse_move.move_down(),
                        KeyCode::Left | KeyCode::Char('a') => mouse_move.move_left(),
                        KeyCode::Right | KeyCode::Char('d') => mouse_move.move_right(),
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()
}
