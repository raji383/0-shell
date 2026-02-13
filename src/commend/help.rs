pub fn help() {
    let yellow = "\x1b[38;5;221m";
    let cyan = "\x1b[38;5;44m";
    //let gray = "\x1b[38;5;245m";
    let purple = "\x1b[38;5;141m";
    let reset = "\x1b[0m";
    println!(
        "\r{}================ My Shell Help ================ {}",
        purple, reset
    );

    println!();
    println!("\r{}Built-in commands:{}\n", yellow, reset);

    println!("\r  {}echo{}   : print text to the terminal", cyan, reset);
    println!("\r  {}cd{}     : change current directory", cyan, reset);
    println!("\r  {}ls{}     : list directory contents", cyan, reset);
    println!("\r  {}pwd{}    : print current directory", cyan, reset);
    println!("\r  {}cat{}    : display file contents", cyan, reset);
    println!("\r  {}cp{}     : copy files", cyan, reset);
    println!("\r  {}mv{}     : move or rename files", cyan, reset);
    println!("\r  {}rm{}     : remove files", cyan, reset);
    println!("\r  {}mkdir{}  : create a new directory", cyan, reset);
    println!("\r  {}exit{}   : exit the shell", cyan, reset);
    println!("\r  {}help{}   : show this help message", cyan, reset);

    println!();
    println!(
        "\r{}================================================ {}",
        purple, reset
    );
}
