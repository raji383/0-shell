use crossterm::{cursor, execute};
use std::env;
use std::path::PathBuf;
use std::io::{self};


pub fn cd(args: &[String]) {
    let current = match env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("cd: {}", e);
            return;
        }
    };

    let target: PathBuf = match args.len() {
        // cd
        0 => match env::var("HOME") {
            Ok(h) => PathBuf::from(h),
            Err(_) => {
                execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();

                eprintln!("cd: HOME not set");
                return;
            }
        },

        // cd -
        1 if args[0] == "-" => match env::var("OLDPWD") {
            Ok(old) => PathBuf::from(old),
            Err(_) => {
                execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();

                eprintln!("cd: OLDPWD not set");
                return;
            }
        },

        // cd ~
        1 if args[0] == "~" => match env::var("HOME") {
            Ok(h) => PathBuf::from(h),
            Err(_) => {
                execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();

                eprintln!("cd: HOME not set");
                return;
            }
        },

        // cd path
        1 => PathBuf::from(&args[0]),

        _ => {
            execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();

            eprintln!("cd: too many arguments");
            return;
        }
    };

    if let Err(_e) = env::set_current_dir(&target) {
        return;
    }

    unsafe {
        env::set_var("OLDPWD", current);
    }
}
