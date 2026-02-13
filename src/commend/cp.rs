use crossterm::{cursor, execute};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn cp(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: cp <source> <destination>");
        return;
    }
    let dst = Path::new(&args[args.len() - 1]);
    for i in 0..(args.len() - 1) {
        let src = Path::new(&args[i]);

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
