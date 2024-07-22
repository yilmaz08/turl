use std::io::{self, Read, Write};

pub fn get_multiline_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input).expect("Failed to read input");
    return input;
}

pub fn get_singleline_input(prompt: String) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input;
}