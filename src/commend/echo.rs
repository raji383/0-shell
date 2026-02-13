use crossterm::{cursor, execute};
use std::io::{self};

pub fn echo(args: &[String]) {
    for (i, arg) in args[0..].iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg);
    }    
        execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
        println!();
    }

