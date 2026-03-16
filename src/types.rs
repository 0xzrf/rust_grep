#[derive(Debug)]
pub enum CharacterClasses {
    Digits,
    Characters,
    PositiveMatch(Vec<char>),
    NegativeMatch(Vec<char>),
    SingleMatch(String),
}

pub struct PatternParser(Vec<CharacterClasses>);

impl From<&String> for PatternParser {
    fn from(pattern: &String) -> Self {
        let mut peek_itterator = pattern.chars().peekable();

        let mut patter_vec: Vec<CharacterClasses> = vec![];

        while peek_itterator.peek().is_some() {
            match peek_itterator.peek() {
                Some(x) if x == &'\\' => {
                    peek_itterator.next();
                    if peek_itterator.peek() == Some(&'d') {
                        patter_vec.push(CharacterClasses::Digits)
                    } else if peek_itterator.peek() == Some(&'w') {
                        patter_vec.push(CharacterClasses::Characters)
                    };
                },
                Some(y) if y == &'[' => {
                    peek_itterator.next();
                    let mut is_neg = false;
                    if peek_itterator.peek() == Some(&'^') {
                        is_neg = true;
                        peek_itterator.next(); // jump to the next value, to start putting it in the Vec<char>
                    }
                    // this is a bracket iterrator, so get all the unique values that're there till `]` occurs
                    let mut find_chars_vec: Vec<char> = vec![];

                    // stops when `]` is encountered
                    while let Some(unique_char) = peek_itterator.next_if(|&x| x != ']') {
                        find_chars_vec.push(unique_char);
                    }

                    peek_itterator.next(); // `]` still remains, so do a .next() to take care of it
                },
                Some(literal) => {},
                None => {},
            }
            peek_itterator.next();
        }

        todo!()
    }
}

impl TryFrom<&String> for CharacterClasses {
    type Error = ();
    fn try_from(option: &String) -> Result<Self, Self::Error> {
        match option.as_str() {
            "\\d" => Ok(CharacterClasses::Digits),
            "\\w" => Ok(CharacterClasses::Characters),
            val if val.starts_with("[^") && val.ends_with("]") => {
                let char_vec = val.chars().collect::<Vec<char>>();

                // we exclude the first 2 values, as they contain `[` and `^` characters, which is not the characters of significance for the operation
                let match_chars = char_vec[2..val.len() - 1].to_vec();

                Ok(CharacterClasses::NegativeMatch(match_chars))
            },
            val if val.starts_with("[") && val.ends_with("]") => {
                let char_vec = val.chars().collect::<Vec<char>>();

                let match_chars = char_vec[1..val.len() - 1].to_vec();

                Ok(CharacterClasses::PositiveMatch(match_chars))
            },
            val if val.chars().count() == 1 => Ok(CharacterClasses::SingleMatch(val.to_string())),
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

    pub fn match_bracket_based_input(input: &str, match_chars: &[char]) -> bool {
        let mut result = false;

        for char in input.chars() {
            if match_chars.contains(&char) {
                result = true;
                break;
            }
        }

        result
    }

    pub fn match_negative_bracket_based_input(input: &str, neg_match_chars: &[char]) -> bool {
        let mut result = false;

        println!("neg match chars: {neg_match_chars:#?}");

        for char in input.chars() {
            if !neg_match_chars.contains(&char) {
                result = true;
                break;
            }
        }

        result
    }


    pub fn match_single_pattern(input: &str, pattern: &str) -> bool {
        input.contains(pattern)
    }
}
