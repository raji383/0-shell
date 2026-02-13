use crate::escape_special_chars;
use crossterm::{cursor, execute};
use std::env;
use std::io::{self};
use std::process;

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => {
            execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
            //  let std = escape_special_chars(&path.to_string_lossy());
            let k = path.to_string_lossy();
            let c: Vec<&str> = k.split_whitespace().collect();
            for (i, v) in c.iter().enumerate() {
                execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
                println!("{}", v.trim_matches('"'));
                if c.len() > 1 {
                    if i == c.len() - 1 {
                        println!()
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            process::exit(1);
        }
    }
}
