use std::io;

use clap::Parser;

use super::{CharacterClasses, GrepError};

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct GrepArgs {
    #[arg(short = 'E')]
    option: String,
}

impl GrepArgs {
    pub fn match_pattern(&self) -> Result<bool, GrepError> {
        let mut input_line = String::new();
        let pattern = &self.option;
        io::stdin().read_line(&mut input_line).unwrap();

        // if CharacterClasses::try_from(&input_line).is_err() {
        //     println!("Invalid error");
        //     return ;
        // }

        todo!()
    }
}
