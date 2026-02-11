use std::fs::File;
use std::io::{self, Read};
use crossterm::{cursor, execute};

pub fn cat(args: &[String]) {
    if args.is_empty() {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();

        println!("{}", input);
        return;
    }

    for filename in args {
        match File::open(filename) {
            Ok(mut file) => {
                let mut content = String::new();
                if let Err(_e) = file.read_to_string(&mut content) {
                    continue;
                }
                execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();

                println!("{}", content);
            }
            Err(e) => {
                execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
                eprintln!("cat: {}: {}", filename, e);
            }
        }
    }
}
