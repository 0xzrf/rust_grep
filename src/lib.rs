use std::{env, io, process};


fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        input_line.contains(pattern)
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

pub fn run() -> Result<(), String> {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }
    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // TODO: Uncomment the code below to pass the first stage
    if match_pattern(&input_line, &pattern) { process::exit(0) } else { process::exit(1) }
}
