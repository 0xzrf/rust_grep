use std::io;

use clap::Parser;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct GrepArgs {
    #[arg(short = 'E')]
    target_value: String,
}

impl GrepArgs {
    pub fn match_pattern(&self) -> bool {
        println!("Write the pattern to match");
        let mut input_line = String::new();
        let pattern = &self.target_value;
        io::stdin().read_line(&mut input_line).unwrap();

        if pattern.chars().count() == 1 {
            input_line.contains(pattern)
        } else {
            panic!("Unhandled pattern: {}", pattern)
        }
    }
}
