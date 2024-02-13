use inquire::{InquireError, Select, Text};
use std::io;

mod mouse_move;

fn main() -> io::Result<()> {
    let q = String::from("Muovere il mouse,Randomizzare,randomize_smoothly");
    let options: Vec<&str> = q.split(",").collect();
    let ans: Result<&str, InquireError> = Select::new("Cosa vuoi fare?", options).prompt();

    match ans {
        Ok("Muovere il mouse") => mouse_move::move_with_keyboard(),
        Ok("Randomizzare") => mouse_move::randomize(),
        Ok("Simulare movimento random") => {
            let ans: Result<String, InquireError> =
                Text::new("Per quanti secondi vuoi randomizzare?")
                    .with_default("10")
                    .prompt();
            let seconds = ans.unwrap().parse::<u64>().unwrap();
            mouse_move::randomize_smoothly(seconds)
        }
        _ => Ok(()),
    }
}
