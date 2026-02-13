use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::{cursor, execute};
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::exit;

pub fn cat(args: &[String]) {
    if args.is_empty() {
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
        let filen = filename;
        match File::open(filen) {
            
            Ok(mut file) => {
                let mut content = String::new();
                if file.read_to_string(&mut content).is_ok() {       
                    println!("{}", content.replace("\n","\r\n"));
                }
            }
            Err(e) => {
                eprintln!("cat: {}: {}", filename, e);
            }
        }
    }
}
