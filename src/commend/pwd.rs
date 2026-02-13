use crossterm::{cursor, execute};
use std::env;
use std::io::{self};
pub fn pwd() {
    match env::current_dir() {
        Ok(path) => {
            execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
            println!("{}", path.display());
        }
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
        }
    }
}
