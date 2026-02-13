use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, execute};
use gg::parse;
use std::io::Write;
use std::{env, io};
fn main() -> std::io::Result<()> {
    let purple = "\x1b[38;5;141m";
    let cyan = "\x1b[38;5;44m";
    let orange = "\x1b[38;5;208m";
    let gray = "\x1b[38;5;245m";
    let blue_cyan = "\x1b[38;2;47;155;178m";
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
    let mut con = false;
    //let mut input = String::new();
    enable_raw_mode()?;
    loop {
        if line.is_empty() {
            execute!(io::stdout(), cursor::MoveToColumn(0),)?;
            match env::current_dir() {
                Ok(path) => {
                    let dir_name = path.file_name().unwrap_or_else(|| path.as_os_str());
                    print!("{}{}{}", blue_cyan, dir_name.to_string_lossy().replace("\n", "\\n"), reset);
                }
                Err(_) => {}
            }
            print!("$ ");
        } else {
            if con {
                execute!(io::stdout(), cursor::MoveToColumn(0),)?;
                print!("> ");
                con = false
            } else {
                line.push_str("\n");
                execute!(io::stdout(), cursor::MoveToColumn(0),)?;
                print!("dquote> ");
            }
        }
        io::stdout().flush().unwrap();
        loop {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        execute!(io::stdout(), cursor::MoveToColumn(0),)?;
                        println!();
                        if line.ends_with('\\') {
                            con = true;
                            line.pop();
                            break;
                        }
                        if parse(&line) {
                            line.clear();
                        }
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
                            line.clear();
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
