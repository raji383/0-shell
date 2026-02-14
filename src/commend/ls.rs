use crossterm::{cursor, execute};
use std::env;
use std::fs;
use std::io::{self};
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Local};

#[derive(Default)]
struct Widths {
    links: usize,
    user: usize,
    group: usize,
    size: usize,
}

// ===== COLORS =====
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub fn ls(args: &[String]) {
    let mut show_all = false; // -a
    let mut long = false; // -l
    let mut classify = false; // -F
    let mut paths: Vec<String> = Vec::new();

    // ===== parse args =====
    for arg in args {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => show_all = true,
                    'l' => long = true,
                    'F' => classify = true,
                    _ => {
                        eprintln!("ls: invalid option '{}'", c);
                        return;
                    }
                }
            }
        } else {
            paths.push(arg.clone());
        }
    }

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    let multi = paths.len() > 1;

    for (i, path) in paths.iter().enumerate() {
        if i > 0 {
            execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
            println!();
        }
        ls_one_path(path.clone(), show_all, long, classify, multi);
    }
}

// =====================================================

fn ls_one_path(mut path: String, show_all: bool, long: bool, classify: bool, print_header: bool) {
    // expand ~
    if path == "~" {
        path = env::var("HOME").unwrap_or_else(|_| {
            eprintln!("ls: HOME environment variable not set");
            ".".to_string()
        });
    }

    let meta = match fs::metadata(&path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("ls: {}: {}", path, e);
            return;
        }
    };

    // ===== single file =====&
    if meta.is_file() || meta.file_type().is_symlink() || meta.file_type().is_char_device()  || meta.file_type().is_block_device(){
        print_entry(
            Path::new(&path).parent().unwrap_or(Path::new(".")),
            Path::new(&path)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .as_ref(),
            &meta,
            long,
            classify,
            &Widths::default(),
        );
        if !long {
            execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
            println!()
        }
        return;
    }

    // println!("{}",path);

    // ===== directory =====
    let mut entries: Vec<_> = match fs::read_dir(&path) {
        Ok(e) => e.flatten().collect(),
        Err(e) => {
            eprintln!("ls: {}: {}", path, e);
            return;
        }
    };

    entries.sort_by(|a, b| {
        let a_name = a.file_name().to_string_lossy().to_lowercase();
        let b_name = b.file_name().to_string_lossy().to_lowercase();

        let a_clean = a_name.trim_start_matches('.');
        let b_clean = b_name.trim_start_matches('.');

        a_clean.cmp(b_clean)
    });
    if print_header {
        execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
        println!("{}:", path);
    }

    // ===== total =====
    if long {
        let mut total_blocks = 0;

        if show_all {
            if let Ok(m) = fs::metadata(&path) {
                total_blocks += m.blocks();
            }
            if let Ok(m) = fs::metadata(Path::new(&path).join("..")) {
                // 512 *8 = 4096  = 4KB
                //16
                //24
                total_blocks += m.blocks();
            }
        }

        for entry in &entries {
            let name = entry.file_name().to_string_lossy().to_string();
            if !show_all && name.starts_with('.') {
                continue;
            }
            if let Ok(m) = entry.metadata() {
                total_blocks += m.blocks();
            }
        }
        execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();
        // 1sector = 512byte
        //1Kb  = 1024 byte
        println!("total {}", total_blocks / 2);
    }

    // ===== compute widths =====
    let mut widths = Widths::default();
    if long {
        for entry in &entries {
            let name = entry.file_name().to_string_lossy().to_string();
            if !show_all && name.starts_with('.') {
                continue;
            }
            if let Ok(meta) = entry.metadata() {
                let ft = meta.file_type();

                widths.links = widths.links.max(meta.nlink().to_string().len());

                let user = users::get_user_by_uid(meta.uid())
                    .map(|u| u.name().to_string_lossy().to_string())
                    .unwrap_or(meta.uid().to_string());
                widths.user = widths.user.max(user.len());

                let group = users::get_group_by_gid(meta.gid())
                    .map(|g| g.name().to_string_lossy().to_string())
                    .unwrap_or(meta.gid().to_string());
                widths.group = widths.group.max(group.len());

                let size_str = if ft.is_block_device() || ft.is_char_device() {
                    let dev = meta.rdev();
                    if dev != 0 {
                        let major = libc::major(dev);
                        let minor = libc::minor(dev);
                        format!("{}, {}", major, minor)
                    } else {
                        "".to_string()
                    }
                } else if ft.is_symlink() {
                    fs::symlink_metadata(entry.path())
                        .map(|m| m.len().to_string())
                        .unwrap_or_default()
                } else {
                    meta.len().to_string()
                };

                widths.size = widths.size.max(size_str.len());
            }
        }
    }

    // ===== print entries =====
    if show_all {
        // .
        if let Ok(m) = fs::metadata(&path) {
            print_entry(&path, ".", &m, long, classify, &widths);
        }
        // ..
        let parent = Path::new(&path).join("..");
        if let Ok(m) = fs::metadata(&parent) {
            print_entry(&path, "..", &m, long, classify, &widths);
        }
    }

    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        if !show_all && name.starts_with('.') {
            continue;
        }
        if let Ok(m) = entry.metadata() {
            print_entry(&path, &name, &m, long, classify, &widths);
        }
    }

    if !long {
        execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();

        println!();
    }
}

// =====================================================

fn print_entry(
    base_path: impl AsRef<Path>,
    name: &str,
    meta: &fs::Metadata,
    long: bool,
    classify: bool,
    widths: &Widths,
) {
    // socket like docker and database
    let mode = meta.permissions().mode();
    let ft = meta.file_type();

    let file_type = if ft.is_dir() {
        'd'
    } else if ft.is_char_device() {
        'c'
    } else if ft.is_block_device() {
        'b'
    } else if ft.is_symlink() {
        'l'
    } else if ft.is_socket() {
        's'
    } else if ft.is_fifo() {
        'p'
    } else {
        '-'
    };

    let mut perms = format!(
        "{}{}{}{}{}{}{}{}{}",
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' },
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' },
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' },
    );
    //Unsized Type  &path ;;;
    let full_path = base_path.as_ref().join(name);
    let acl_char = if has_acl(&full_path) { '+' } else { ' ' };
    perms.push(acl_char);

    let links = meta.nlink();

    let user = users::get_user_by_uid(meta.uid())
        .map(|u| u.name().to_string_lossy().to_string())
        .unwrap_or(meta.uid().to_string());

    let group = users::get_group_by_gid(meta.gid())
        .map(|g| g.name().to_string_lossy().to_string())
        .unwrap_or(meta.gid().to_string());

    let size = if ft.is_block_device() || ft.is_char_device() {
        let dev = meta.rdev();
        if dev != 0 {
            let major = (dev >> 8) & 0xfff;
            let minor = (dev & 0xff) | ((dev >> 12) & 0xfff00);
            format!("{}, {}", major, minor)
        } else {
            "".to_string()
        }
    } else if ft.is_symlink() {
        fs::symlink_metadata(&full_path)
            .map(|m| m.len().to_string())
            .unwrap_or_default()
    } else {
        meta.len().to_string()
    };

    let mtime = meta.mtime();
    let system_time = UNIX_EPOCH + Duration::from_secs((mtime + 3600) as u64);
    let datetime: DateTime<Local> = system_time.into();
    let date = datetime.format("%b %d %H:%M");

    let is_symlink = ft.is_symlink();
    let is_dir = meta.is_dir();
    let is_exec = mode & 0o111 != 0;
    let is_socket = ft.is_socket();
    let is_fifo = ft.is_fifo();

    let colored = if is_symlink {
        let mut display = name.replace("\n", "'$\\n'");

        if classify {
            display.push('@');
        }
        if let Ok(target) = fs::read_link(&full_path) {
            let target_str = target.to_string_lossy();
            format!("{CYAN}{display}{RESET} -> {target_str}")
        } else {
            format!("{CYAN}{display}{RESET} -> {RED}(broken){RESET}")
        }
    } else {
        let mut display = name.replace("\n", "'$\\n'");

        if classify {
            if is_dir {
                display.push('/');
            } else if is_exec {
                display.push('*');
            } else if is_fifo {
                display.push('|');
            } else if is_socket {
                display.push('=');
            }
        }

        if is_dir {
            format!("{BLUE}{display}{RESET}")
        } else if is_exec {
            format!("{GREEN}{display}{RESET}")
        } else {
            let display = name.replace("\n", "'$\\n'");

            display
        }
    };

    if long {
        execute!(io::stdout(), cursor::MoveToColumn(0),).unwrap();

        println!(
            "{}{} {:>links$} {:<user$} {:<group$} {:>size$} {} {}",
            file_type,
            perms,
            links,
            user,
            group,
            size,
            date,
            colored,
            links = widths.links,
            user = widths.user,
            group = widths.group,
            size = widths.size,
        );
    } else {
        print!("{}  ", colored);
    }
}
//Extended Attribute   more info in file
fn has_acl(path: &Path) -> bool {
    if let Ok(attrs) = xattr::list(path) {
        for attr in attrs {
            if attr.to_string_lossy().starts_with("system.posix_acl") {
                return true;
            }
        }
    }
    false
}
// dev virtual file size 0
