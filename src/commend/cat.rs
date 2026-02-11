use std::fs::File;
use std::io::{self, Read};

pub fn cat(args: &[String]) {
    if args.is_empty() {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        print!("{}", input);
        return;
    }

    for filename in args {
        match File::open(filename) {
            Ok(mut file) => {
                let mut content = String::new();
                if let Err(e) = file.read_to_string(&mut content) {
                    eprintln!("cat: {}: {}", filename, e);
                    continue;
                }
                print!("{}", content);
            }
            Err(e) => {
                eprintln!("cat: {}: {}", filename, e);
            }
        }
    }
}
