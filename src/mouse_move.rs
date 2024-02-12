use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use mouse_rs::{types::Point, Mouse};
use rand::prelude::*;
use std::io::{self, stdout, Write};

const MOUSE_MOVE_DISTANCE: i32 = 100;

pub struct MouseMove {
    x: i32,
    y: i32,
}

impl MouseMove {
    pub fn new(x: i32, y: i32) -> MouseMove {
        MouseMove { x, y }
    }

    fn get_current_position(&self) -> Result<Point, Box<dyn std::error::Error>> {
        let mouse = Mouse::new();
        let pos = mouse.get_position().unwrap();
        println!("X = {}, Y = {}", pos.x, pos.y);
        Ok(pos)
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn move_up(&mut self) {
        let mouse = Mouse::new();

        let position = self.get_current_position();
        let position = position.unwrap();

        mouse
            .move_to(position.x, position.y - MOUSE_MOVE_DISTANCE)
            .unwrap();
        self.set_position(position.x, position.y - MOUSE_MOVE_DISTANCE);
    }

    pub fn move_down(&mut self) {
        let mouse = Mouse::new();

        let position = self.get_current_position();
        let position = position.unwrap();

        mouse
            .move_to(position.x, position.y + MOUSE_MOVE_DISTANCE)
            .unwrap();

        self.set_position(position.x, position.y + MOUSE_MOVE_DISTANCE);
    }
    pub fn move_left(&mut self) {
        let mouse = Mouse::new();

        let position = self.get_current_position();
        let position = position.unwrap();

        mouse
            .move_to(position.x - MOUSE_MOVE_DISTANCE, position.y)
            .unwrap();
        self.set_position(position.x - MOUSE_MOVE_DISTANCE, position.y);
    }
    pub fn move_right(&mut self) {
        let mouse = Mouse::new();

        let position = self.get_current_position();
        let position = position.unwrap();

        mouse
            .move_to(position.x + MOUSE_MOVE_DISTANCE, position.y)
            .unwrap();
        self.set_position(position.x + MOUSE_MOVE_DISTANCE, position.y);
    }
}

pub fn move_with_keyboard() -> io::Result<()> {
    enable_raw_mode()?;

    let mut mouse_move = MouseMove::new(0, 0);
    loop {
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Esc | KeyCode::Char('q') => break,
                _ => {
                    println!("You pressed: {:?}", event.code);
                    stdout().flush().unwrap();
                    match event.code {
                        KeyCode::Up | KeyCode::Char('w') => mouse_move.move_up(),
                        KeyCode::Down | KeyCode::Char('s') => mouse_move.move_down(),
                        KeyCode::Left | KeyCode::Char('a') => mouse_move.move_left(),
                        KeyCode::Right | KeyCode::Char('d') => mouse_move.move_right(),
                        _ => println!("You pressed: {:?}", event.code),
                    }
                }
            }
        }
    }

    disable_raw_mode()
}

pub fn randomize() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    let mouse = Mouse::new();
    let mut mouse_move = MouseMove::new(0, 0);

    enable_raw_mode()?;

    loop {
        let x: i32 = rng.gen_range(0..1000);
        let y: i32 = rng.gen_range(0..1000);
        mouse.move_to(x, y).unwrap();
        mouse_move.set_position(x, y);

        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Esc | KeyCode::Char('q') => break,
                _ => println!("You pressed: {:?}", event.code),
            }
        }
    }

    disable_raw_mode()
}

// create public function to call from main.rs
// to move mouse smoothly randomly in the screen

pub fn randomize_smoothly(seconds: i32) -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    let mouse = Mouse::new();
    let mut mouse_move = MouseMove::new(0, 0);
    enable_raw_mode()?;

    for s in 0..seconds {
        println!("Seconds: {}", seconds - s);
        // get before moving position
        let position = mouse_move.get_current_position().unwrap();
        let x: i32 = rng.gen_range(position.x - 50..position.x + 50);
        let y: i32 = rng.gen_range(position.y - 50..position.y + 50);
        mouse.move_to(x, y).unwrap();
        mouse_move.set_position(x, y);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    disable_raw_mode()
}
