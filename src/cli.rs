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

        io::stdin().read_line(&mut input_line).unwrap();

        if let Ok(option) = CharacterClasses::try_from(&self.option) {
            match option {
                CharacterClasses::Characters => {
                    Ok(CharacterClasses::string_contains_character_class(&input_line))
                },
                CharacterClasses::Digits => {
                    Ok(CharacterClasses::string_contains_digit_class(&input_line))
                },
                CharacterClasses::MultiMatch(pattern) => {
                    Ok(CharacterClasses::match_bracket_based_input(&input_line, &pattern))
                },
                CharacterClasses::SingleMatch(pattern) => {
                    Ok(CharacterClasses::match_single_pattern(&input_line, &pattern))
                },
            }
        } else {
            Err(GrepError::InvalidOptionProvided)
        }
    }
}
