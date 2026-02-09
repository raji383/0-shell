pub mod commend;

pub fn parse(l: &str) {
    let arg = l.split("&");
    let mut le = Vec::new();
    for s in arg {
        if s.is_empty() {
            continue;
        }
        let mut args = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;

        for c in s.chars() {
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
    for i in le {
        aplye(i)
    }
}
fn aplye(arg: Vec<String>) {
    if arg.len() == 0 {
        return;
    }
    match arg[0].as_str() {
        "ls" => {}
        _ => {
            println!("commend not found {}",arg[0])
        },
    }
}
