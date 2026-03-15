use codecrafters_grep::run;
// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if let Err(_err) = run() {
        std::process::exit(1)
    }
}
