use std::io;

use clap::Parser;

use super::{GrepError, PatternParser};

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct GrepArgs {
    #[arg(short = 'E')]
    option: String,
}

impl GrepArgs {
    pub fn match_pattern(&self) -> Result<bool, GrepError> {
        let mut input_line = String::new();

        io::stdin().read_line(&mut input_line).unwrap();

        todo!()
    }
}
