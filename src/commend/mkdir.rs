use crossterm::{cursor, execute};
use std::fs;
use std::path::Path;

pub fn mkdir(args: &[String]) {
    if args.is_empty() {
        execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
        eprintln!("mkdir: missing data mon ami");
        return;
    }

    for name in args {
        let path = Path::new(name);

        if let Err(e) = fs::create_dir(path) {
            execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
            eprintln!("mkdir: cannot create directory mo ami '{}': {}", name, e);
        }
    }
}
