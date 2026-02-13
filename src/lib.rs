use crossterm::terminal::disable_raw_mode;
pub mod commend;

pub fn parse(l: &str) -> bool {
    /*if l.ends_with('\\') {
        return false;
    }*/

    let arg = l.split("&");
    let mut le = Vec::new();
    let mut in_quotes = false;
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
    let reset = "\x1b[0m";

    match arg[0].as_str() {
        "exit" => {
            println!("\r{}Exiting the shell. Goodbye!{}", green, reset);
            if let Err(e) = disable_raw_mode() {
                eprintln!("\rFailed to disable raw mode: {}", e);
            }
            std::process::exit(0);
        }

        "help" => crate::commend::help::help(),

        "echo" => crate::commend::echo::echo(&arg[1..]),

        "pwd" => {
            crate::commend::pwd::pwd();
        }

        "cd" => crate::commend::cd::cd(&arg[1..]),

        "ls" => crate::commend::ls::ls(&arg[1..]),

        "cat" => crate::commend::cat::cat(&arg[1..]),

        "mkdir" => crate::commend::mkdir::mkdir(&arg[1..]),

        "rm" => crate::commend::rm::rm(&arg[1..]),

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
