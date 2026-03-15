use std::process::exit;

use codecrafters_grep::run;
// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    match run() {
        Ok(_) => exit(0),
        Err(e) => {
            println!("Error occurred: {e}");
            exit(1);
        },
    }
}
