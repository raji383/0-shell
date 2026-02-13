use crate::unescape;
use crossterm::{cursor, execute};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn cp(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: cp <source> <destination>");
        return;
    }
    let mut name = unescape(&args[args.len() - 1]);
    let c = Path::new(&name);
    if c.exists() {
    } else {
        name = args[args.len() - 1].clone();
    }

    let dst = Path::new(&name);
    for i in 0..(args.len() - 1) {
        let mut name = unescape(&args[i]);

        let c = Path::new(&name);
        if c.exists() {
        } else {
            name = args[i].clone();
        }

        let src = Path::new(&name);

        if !src.exists() {
            eprintln!("Error: source file does not exist");
            return;
        }

        // If destination is directory, append filename
        let final_dst: PathBuf = if dst.is_dir() {
            match src.file_name() {
                Some(name) => dst.join(name),
                None => {
                    eprintln!("Error: invalid source filename");
                    return;
                }
            }
        } else {
            dst.to_path_buf()
        };

        match fs::copy(src, &final_dst) {
            Ok(_) => {
                if let Err(e) = execute!(io::stdout(), cursor::MoveToColumn(0)) {
                    eprintln!("Terminal error: {}", e);
                }
                println!("Copied {} -> {}", src.display(), final_dst.display());
            }
            Err(e) => eprintln!("Error copying file: {}", e),
        }
    }
}
