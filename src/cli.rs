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
            }
        } else {
            Ok(Self::match_input_contains_val(&input_line, &self.option))
        }
    }

    pub fn match_input_contains_val(input: &str, pattern: &str) -> bool {
        if pattern.chars().count() == 1 {
            input.contains(pattern)
        } else {
            panic!("Invalid pattern")
        }
    }
}
