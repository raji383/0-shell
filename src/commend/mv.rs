use std::{
    fs,
    path::{Path, PathBuf},
};
use crate::unescape;

pub fn mv(args: &[String]) {
    if args.len() == 0 {
        eprintln!("mv: missing file operand");
        return;
    } else if args.len() == 3 {
        eprintln!("mv: missing destination file operand after {}", args[1]);
        return;
    }
    let mut name = unescape(&args[args.len() - 1]);
    let c = Path::new(&name);
    if c.exists() {
    } else {
        name = args[args.len() - 1].clone();
    }

    let folder = Path::new(&name);
    for i in 0..(args.len() - 1) {
        let mut name = unescape(&args[i]);

        let c = Path::new(&name);
        if c.exists() {
        } else {
            name = args[i].clone();
        }
        let src = Path::new(&name);
        if !src.exists() {
            eprintln!(
                "mv: cannot stat '{}': No such file or directory",
                src.display()
            );
            continue;
        }
        let mut exist = true;
        let final_dst: PathBuf = if folder.is_dir() {
            let name = src.file_name().expect("Invalid source");
            exist = false;
            folder.join(name)
        } else {
            folder.to_path_buf()
        };

        if final_dst.exists() && !exist {
            eprintln!(
                "mv: cannot move '{}': destination '{}' already exists",
                src.display(),
                final_dst.display()
            );
            continue;
        }

        if let Err(e) = fs::rename(src, &final_dst) {
            eprintln!(
                "mv: cannot move '{}' to '{}': {}",
                src.display(),
                final_dst.display(),
                e
            );
        }
    }
}
