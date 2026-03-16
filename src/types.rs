#[derive(Debug, PartialEq)]
pub enum CharacterClasses {
    Digits,
    Characters,
    PositiveMatch(Vec<char>),
    NegativeMatch(Vec<char>),
    SingleMatch(String),
    Literal(String),
    WhiteSpace,
}

#[derive(Debug)]
pub struct PatternParser(Vec<CharacterClasses>);

impl From<&str> for PatternParser {
    fn from(pattern: &str) -> Self {
        let mut peek_itterator = pattern.chars().peekable();

        let mut pattern_vec: Vec<CharacterClasses> = vec![];

        while peek_itterator.peek().is_some() {
            match peek_itterator.peek() {
                Some(x) if x == &'\\' => {
                    peek_itterator.next();
                    if peek_itterator.peek() == Some(&'d') {
                        pattern_vec.push(CharacterClasses::Digits)
                    } else if peek_itterator.peek() == Some(&'w') {
                        pattern_vec.push(CharacterClasses::Characters)
                    };
                },
                Some(y) if y == &'[' => {
                    peek_itterator.next();
                    let mut is_neg = false;
                    if peek_itterator.peek() == Some(&'^') {
                        is_neg = true;
                        peek_itterator.next(); // skip `^` and jump to the next value, to start putting it in the Vec<char>
                    }
                    // this is a bracket iterrator, so get all the unique values that're there till `]` occurs
                    let mut find_chars_vec: Vec<char> = vec![];

                    // stops when `]` is encountered
                    while let Some(unique_char) = peek_itterator.next_if(|&x| x != ']') {
                        find_chars_vec.push(unique_char);
                    }

                    if is_neg {
                        pattern_vec.push(CharacterClasses::NegativeMatch(find_chars_vec));
                    } else {
                        pattern_vec.push(CharacterClasses::PositiveMatch(find_chars_vec));
                    }
                },
                Some(space) if space == &' ' => {
                    pattern_vec.push(CharacterClasses::WhiteSpace);
                },
                Some(_literal) => {
                    let mut literal_char_vec: Vec<char> = vec![];

                    while let Some(char) = peek_itterator
                        .next_if(|char| char.ne(&'\\') || char.ne(&'[') || char.ne(&' '))
                    {
                        literal_char_vec.push(char);
                    }

                    pattern_vec.push(CharacterClasses::Literal(
                        literal_char_vec.into_iter().collect::<String>(),
                    ));
                },
                None => {},
            }
            peek_itterator.next();
        }

        PatternParser(pattern_vec)
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


#[cfg(test)]
pub mod pattern_parser_tests {

    use super::*;
    #[test]
    pub fn test_convert_string_to_pattern_parser() {
        let pattern_str = "\\d\\d";
        let expected_vec = vec![CharacterClasses::Digits, CharacterClasses::Digits];

        let parsed_pattern = PatternParser::from(pattern_str).0;

        assert_eq!(expected_vec, parsed_pattern);

        let pattern_str = "\\d \\d"; // handle whitespace case scenario
        let expected_vec =
            vec![CharacterClasses::Digits, CharacterClasses::WhiteSpace, CharacterClasses::Digits];
        let parsed_pattern = PatternParser::from(pattern_str).0;

        assert_eq!(expected_vec, parsed_pattern);

        let pattern_str = "\\d \\d[abc]";

        let expected_vec = vec![
            CharacterClasses::Digits,
            CharacterClasses::WhiteSpace,
            CharacterClasses::Digits,
            CharacterClasses::PositiveMatch(vec!['a', 'b', 'c']),
        ];

        let parsed_pattern = PatternParser::from(pattern_str).0;

        assert_eq!(expected_vec, parsed_pattern);

        let pattern_str = "\\d \\d[abc]literal_val";

        let expected_vec = vec![
            CharacterClasses::Digits,
            CharacterClasses::WhiteSpace,
            CharacterClasses::Digits,
            CharacterClasses::PositiveMatch(vec!['a', 'b', 'c']),
            CharacterClasses::Literal("literal_val".to_string()),
        ];

        let parsed_pattern = PatternParser::from(pattern_str).0;

        assert_eq!(expected_vec, parsed_pattern);
    }
}
