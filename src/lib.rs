use crossterm::terminal::disable_raw_mode;
use crossterm::{cursor, execute};
use std::io;
pub mod commend;

pub fn parse(l: &str) -> bool {
    /*if l.ends_with('\\') {
        return false;
    }*/

    let arg = l.split("&");
    let mut le = Vec::new();
    let mut in_quotes = true;
    for s in arg {
        if s.is_empty() {
            continue;
        }
        let mut args = Vec::new();
        let mut current = String::new();
        in_quotes = false;
        let src = s.replace("\\\"", "");
        for c in src.chars() {
            match c {
                '"' => in_quotes = !in_quotes,
                ' ' if !in_quotes => {
                    if !current.is_empty() {
                        args.push(current.clone());
                        current.clear();
                    }
                }
                _ => current.push(c),
            }
        }

        if !current.is_empty() {
            args.push(current);
        }
        le.push(args);
    }
    if in_quotes {
        return false;
    }
    for i in le {
        aplye(i)
    }
    true
}
fn aplye(arg: Vec<String>) {
    if arg.len() == 0 {
        return;
    }
    let green = "\x1b[38;5;82m";
    let yellow = "\x1b[38;5;221m";
    let cyan = "\x1b[38;5;44m";
    //let gray = "\x1b[38;5;245m";
    let purple = "\x1b[38;5;141m";
    let reset = "\x1b[0m";

    match arg[0].as_str() {
        "exit" => {
            println!("{}Exiting the shell. Goodbye!{}", green, reset);
            disable_raw_mode().unwrap();
            std::process::exit(0);
        }

        "help" => {
            println!(
                "{}================ My Shell Help ================{}",
                purple, reset
            );
            execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
            println!("{}Built-in commands:{}", yellow, reset);
            execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
            println!(
                "{}  echo, cd, ls, pwd, cat, cp, rm, mv, mkdir, exit, help{}",
                cyan, reset
            );
            execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
            println!(
                "{}================================================{}",
                purple, reset
            );
        }

        "echo" => {}

        "pwd" => {
            crate::commend::pwd::pwd();
        }

        "cd" => {}

        "ls" => {
            crate::commend::ls::ls(&arg[1..])
        }

        "cat" => {
             crate::commend::cat::cat(&arg[1..])

        }
        

        "mkdir" => {}

        "rm" => {}

        "cp" => {
            crate::commend::cp::cp(&arg[1..]);
        }

        "mv" => {
            crate::commend::mv::mv(&arg[1..]);
        }

        _ => {
            eprintln!("Command not found: {}", arg[0]);
        }
    }
}
