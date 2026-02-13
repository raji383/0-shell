use crossterm::{cursor, execute};
use std::fs;
use std::path::Path;
use std::io::{self};
use crate::unescape;


pub fn mkdir(args: &[String]) {
    if args.is_empty() {
        execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
        eprintln!("mkdir: missing data mon ami");
        return;
    }
 println!("{:?}",args);
   for name in args {
    let cleaned = name.trim_matches('"');

    let fixed = unescape(cleaned);

    let path = Path::new(&fixed);

    if let Err(e) = fs::create_dir(path) {
        execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
        eprintln!("mkdir: cannot create directory '{}': {}", fixed, e);
    }
}

}


