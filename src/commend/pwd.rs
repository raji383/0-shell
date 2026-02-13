use crate::escape_special_chars;
use crossterm::{cursor, execute};
use std::env;
use std::io::{self};
pub fn pwd() {
    match env::current_dir() {
        Ok(path) => {
            execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
            //  let std = escape_special_chars(&path.to_string_lossy());
            let k = path.to_string_lossy();
            let c: Vec<&str> = k.split('\n').collect();
            println!("{:?}",c);
            for (_i, v) in c.iter().enumerate() {
                execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
                println!("{}", v.trim_matches('"'))
            }
        }
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
        }
    }
}
