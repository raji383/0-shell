use std::{
    fs,
    path::{Path, PathBuf},
};
pub fn mv(args: &[String]) {
    if args.len() == 1 {
        eprintln!("mv: missing file operand");
        return;
    } else if args.len() == 2 {
        eprintln!("mv: missing destination file operand after {}", args[1]);
        return;
    }
    let folder = Path::new(&args[args.len() - 1]);
    for i in 1..(args.len() - 1) {
        let src = Path::new(&args[i]);

        if !src.exists() {
            eprintln!(
                "mv: cannot stat '{}': No such file or directory",
                src.display()
            );
            continue;
        }

        let final_dst: PathBuf = if folder.is_dir() {
            let name = src.file_name().expect("Invalid source");
            folder.join(name)
        } else {
            folder.to_path_buf()
        };

        if final_dst.exists() {
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
