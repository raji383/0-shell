use crossterm::{cursor, execute};
use std::fs;
use std::io;
use std::path::Path;

pub fn rm(args: &[String]) {
    if args.is_empty() {
        let _ = execute!(io::stdout(), cursor::MoveToColumn(0));
        eprintln!("rm: missing operand");
        return;
    }

    let mut recursive = false;
    let mut force = false;
    let mut files = Vec::new(); 

    for arg in args {
        if arg.starts_with('-') && arg.len() > 1 {
            for c in arg.chars().skip(1) {
                match c {
                    'r' | 'R' => recursive = true,
                    'f' => force = true,
                    _ => {
                        let _ = execute!(io::stdout(), cursor::MoveToColumn(0));
                        eprintln!("rm: invalid option -- '{}'", c);
                        return;
                    }
                }
            }
        } else {
            files.push(arg);
        }
    }

    if files.is_empty() {
        if !force {
            let _ = execute!(io::stdout(), cursor::MoveToColumn(0));
            eprintln!("rm: missing operand");
        }
        return;
    }

    for name in files {
        let path = Path::new(name);

        if !path.exists() {
            if !force {
                let _ = execute!(io::stdout(), cursor::MoveToColumn(0));
                eprintln!("rm: cannot remove '{}': No such file or directory", name);
            }
            continue;
        }

        let res = if path.is_dir() {
            if recursive {
                fs::remove_dir_all(path)
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "Is a directory"))
            }
        } else {
            fs::remove_file(path)
        };

        if let Err(e) = res {
            if !force {
                let _ = execute!(io::stdout(), cursor::MoveToColumn(0));
                eprintln!("rm: cannot remove '{}': {}", name, e);
            }
        }
    }
}