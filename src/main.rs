use inquire::{InquireError, Select};
use std::io;

mod mouse_move;

fn main() -> io::Result<()> {
    let q = String::from("Muovere il mouse,Randomizzare,randomize_smoothly");
    let options: Vec<&str> = q.split(",").collect();
    let ans: Result<&str, InquireError> = Select::new("Cosa vuoi fare?", options).prompt();

    match ans {
        Ok("Muovere il mouse") => mouse_move::move_with_keyboard(),
        Ok("Randomizzare") => mouse_move::randomize(),
        Ok("randomize_smoothly") => mouse_move::randomize_smoothly(10),
        _ => Ok(()),
    }
}
