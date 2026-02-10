use crossterm::{cursor, execute};
use std::fs;
use std::io::{self};
pub fn cp(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: to file at laste");
        return;
    };
    let src = &args[1];
    let dst = &args[2];

    match fs::copy(src, dst) {
        Ok(_) => {
            execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
            println!("Copied {} -> {}", src, dst)
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
