use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute};
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::exit;

pub fn cat(args: &[String]) {
    if args.is_empty() {
        //enable_raw_mode().unwrap();
        let mut line = String::new();

        loop {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char(c) => {
                        // Ctrl+D
                        if c == 'd' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            println!();
                            exit(0);
                        }

                        // Ctrl+C
                        if c == 'c' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            line.clear();
                            execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
                            println!();
                            return;
                        }
                        line.push(c);
                        print!("{}", c);
                        io::stdout().flush().unwrap();
                    }

                    KeyCode::Enter => {
                        execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
                        println!();
                        println!("{}", line);
                        execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
                        line.clear();
                    }

                    _ => {}
                }
            }
        }
    }

    // ===== File mode =====
    for filename in args {
        match File::open(filename) {
            Ok(mut file) => {
                let mut content = String::new();
                if file.read_to_string(&mut content).is_ok() {
                    println!("{}", content);
                }
            }
            Err(e) => {
                eprintln!("cat: {}: {}", filename, e);
            }
        }
    }
}
