use crossterm::{cursor, execute};
use std::io::{self, Write};

pub fn echo(args: &[String]) {
    let mut newline = true;
    let mut start = 0;

    // check -n
    if !args.is_empty() && args[0] == "-n" {
        newline = false;
        start = 1;
        print!("{}", args[1]);
    }

    // print args
    for (i, arg) in args[start..].iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
        print!("{}", arg);
    }

    // newline
    if newline {
        execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
        println!();
    } else {
        io::stdout().flush().unwrap();
    }
}
