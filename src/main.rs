use mouse_rs::{types::Point, Mouse};
use std::io::{self, Read};

// fn move_and_press() {
//     let mouse = Mouse::new();
//     mouse.move_to(500, 500).expect("Unable to move mouse");
//     mouse.press(&Keys::RIGHT).expect("Unable to press button");
//     mouse
//         .release(&Keys::RIGHT)
//         .expect("Unable to release button");
// }

fn get_current_position() -> Result<Point, Box<dyn std::error::Error>> {
    let mouse = Mouse::new();
    let pos = mouse.get_position().unwrap();
    println!("X = {}, Y = {}", pos.x, pos.y);
    Ok(pos)
}

fn move_up() {
    let mouse = Mouse::new();

    let position = get_current_position();
    let position = position.unwrap();
    mouse.move_to(position.x, position.y - 100).unwrap();
}

fn move_down() {
    let mouse = Mouse::new();

    let position = get_current_position();
    let position = position.unwrap();
    mouse.move_to(position.x, position.y + 100).unwrap();
}
fn move_left() {
    let mouse = Mouse::new();

    let position = get_current_position();
    let position = position.unwrap();
    mouse.move_to(position.x - 100, position.y).unwrap();
}
fn move_right() {
    let mouse = Mouse::new();

    let position = get_current_position();
    let position = position.unwrap();
    mouse.move_to(position.x + 100, position.y).unwrap();
}

fn main() -> io::Result<()> {
    // Create a buffer to store user input
    let mut buffer = [0; 1];

    // Use stdin to read input from the terminal
    let stdin = io::stdin();

    // Get the handle to the input stream
    let mut handle = stdin.lock();

    println!("Type something (press 'q' to quit):");

    loop {
        // Read a single byte from the input stream

        let bytes_read = handle.read(&mut buffer);

        // Check if any bytes were read
        if bytes_read.is_err() {
            println!("Error reading input");
            break;
        } else if bytes_read.unwrap() == 0 {
            println!("No bytes read");
            break;
        }
        // Convert the byte to a character
        let input_char = buffer[0] as char;

        // Print the character
        println!("You typed: {}", input_char);

        // Check if the character is 'q', if so, quit the loop
        if input_char == 'q' {
            break;
        } else if input_char == 'w' {
            move_up();
        } else if input_char == 's' {
            move_down();
        } else if input_char == 'a' {
            move_left();
        } else if input_char == 'd' {
            move_right();
        }
    }

    Ok(())

    // move_and_press();
}
