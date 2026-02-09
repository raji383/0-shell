pub mod commend;

pub fn parse(l: &str) {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in l.chars() {
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
    println!("{:?}",args)
}
