#[derive(Debug)]
pub enum CharacterClasses {
    Digits,
    Characters,
}

impl TryFrom<&String> for CharacterClasses {
    type Error = ();
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "\\d" => Ok(CharacterClasses::Digits),
            "\\w" => Ok(CharacterClasses::Characters),
            _ => Err(()),
        }
    }
}

impl CharacterClasses {
    pub fn string_contains_digit_class(target_value: &str) -> bool {
        let mut result = false;
        for char in target_value.chars() {
            if char.is_ascii_digit() {
                result = true;
                break;
            }
        }

        result
    }

    pub fn string_contains_character_class(target_value: &str) -> bool {
        let mut result = false;
        for char in target_value.chars() {
            if char.is_alphanumeric() || char == '_' {
                result = true;
                break;
            }
        }

        result
    }
}
