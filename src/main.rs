use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute};
use gg::parse;
use std::io;
use std::io::Write;
fn main() -> std::io::Result<()> {
    let purple = "\x1b[38;5;141m";
    let cyan = "\x1b[38;5;44m";
    let orange = "\x1b[38;5;208m";
    let gray = "\x1b[38;5;245m";
    let reset = "\x1b[0m";

    println!("{}===================================={}", purple, reset);
    println!("{}       WELCOME TO 0-SHELL{}", cyan, reset);
    println!("{}   The minimal Rust command shell{}", gray, reset);

    println!(
        r"{}
                                         _              _   _ 
                   ___                  | |            | | | | 
                  / _ \   ______   ___  | |__     ___  | | | | 
                 | | | | |______| / __| |  _ \   / _ \ | | | | 
                 | |_| |          \__ \ | | | | |  __/ | | | | 
                  \___/           |___/ |_| |_|  \___| |_| |_| 
{}",
        orange, reset
    );

    println!("{}===================================={}", purple, reset);
    println!("{}Type 'help' to see built-in commands{}\n", cyan, reset);
    let mut line = String::new();
    //let mut input = String::new();
    enable_raw_mode()?;
    loop {
        if line.is_empty() {
            execute!(io::stdout(), cursor::MoveToColumn(0),)?;
            print!("$ ");
        }
        io::stdout().flush().unwrap();
        loop {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        execute!(io::stdout(), cursor::MoveToColumn(0),)?;
                        println!();
                        parse(&line);
                        line.clear();
                        break;
                    }
                    KeyCode::Backspace => {
                        if !line.is_empty() {
                            line.pop();
                            print!("\u{8} \u{8}");
                            io::stdout().flush().unwrap();
                        }
                    }
                    KeyCode::Char(c) => {
                        if c == 'd' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            disable_raw_mode()?;
                            return Ok(());
                        }
                        if c == 'c' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            execute!(io::stdout(), cursor::MoveToColumn(0),)?;
                            println!();
                            break;
                        }
                        line.push(c);
                        print!("{}", c);
                        io::stdout().flush().unwrap();
                    }

                    _ => {} // arrows ignored
                }
            }
        }
    }
    //disable_raw_mode()?;
}
