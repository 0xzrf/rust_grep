#[derive(Debug, PartialEq)]
pub enum CharacterClasses {
    Digits,
    Characters,
    PositiveMatch(Vec<char>),
    NegativeMatch(Vec<char>),
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
            let mut skip_next = true;
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
                        .next_if(|char| char.ne(&'\\') && char.ne(&'[') && char.ne(&' '))
                    {
                        literal_char_vec.push(char);
                    }

                    pattern_vec.push(CharacterClasses::Literal(
                        literal_char_vec.into_iter().collect::<String>(),
                    ));
                    skip_next = false;
                },
                None => {},
            }
            if skip_next {
                peek_itterator.next();
            }
        }

        PatternParser(pattern_vec)
    }
}


#[cfg(test)]
pub mod pattern_parser_tests {
    use super::*;

    pub fn assert_equality_test(pattern_str: &str, expected_pattern: Vec<CharacterClasses>) {
        let parsed_pattern = PatternParser::from(pattern_str).0;

        assert_eq!(
            expected_pattern, parsed_pattern,
            "The pattern didn't parse into expected pattern: {pattern_str}"
        );
    }

    #[test]
    pub fn test_convert_string_to_pattern_parser() {
        let pattern_str = "\\d\\d";
        let expected_pattern = vec![CharacterClasses::Digits, CharacterClasses::Digits];

        assert_equality_test(pattern_str, expected_pattern);

        let pattern_str = "\\d \\d"; // handle whitespace case scenario
        let expected_pattern =
            vec![CharacterClasses::Digits, CharacterClasses::WhiteSpace, CharacterClasses::Digits];
        assert_equality_test(pattern_str, expected_pattern);
        let pattern_str = "\\d \\d[abc]";

        let expected_pattern = vec![
            CharacterClasses::Digits,
            CharacterClasses::WhiteSpace,
            CharacterClasses::Digits,
            CharacterClasses::PositiveMatch(vec!['a', 'b', 'c']),
        ];

        assert_equality_test(pattern_str, expected_pattern);
        let pattern_str = "\\d \\d[abc]literal_val";

        let expected_pattern = vec![
            CharacterClasses::Digits,
            CharacterClasses::WhiteSpace,
            CharacterClasses::Digits,
            CharacterClasses::PositiveMatch(vec!['a', 'b', 'c']),
            CharacterClasses::Literal("literal_val".to_string()),
        ];

        assert_equality_test(pattern_str, expected_pattern);
        let pattern_str = "\\d apple";

        let expected_pattern = vec![
            CharacterClasses::Digits,
            CharacterClasses::WhiteSpace,
            CharacterClasses::Literal("apple".to_string()),
        ];

        assert_equality_test(pattern_str, expected_pattern);
        let pattern_str = "\\d\\d\\d apples";

        let expected_pattern = vec![
            CharacterClasses::Digits,
            CharacterClasses::Digits,
            CharacterClasses::Digits,
            CharacterClasses::WhiteSpace,
            CharacterClasses::Literal("apples".to_string()),
        ];

        assert_equality_test(pattern_str, expected_pattern);
        let pattern_str = "\\d \\w\\w\\ws";

        let expected_pattern = vec![
            CharacterClasses::Digits,
            CharacterClasses::WhiteSpace,
            CharacterClasses::Characters,
            CharacterClasses::Characters,
            CharacterClasses::Characters,
            CharacterClasses::Literal("s".to_string()),
        ];

        assert_equality_test(pattern_str, expected_pattern);

        let pattern_str = "\\d \\w\\w\\ws[abc][^abc]abc";

        let expected_pattern = vec![
            CharacterClasses::Digits,
            CharacterClasses::WhiteSpace,
            CharacterClasses::Characters,
            CharacterClasses::Characters,
            CharacterClasses::Characters,
            CharacterClasses::Literal("s".to_string()),
            CharacterClasses::PositiveMatch(vec!['a', 'b', 'c']),
            CharacterClasses::NegativeMatch(vec!['a', 'b', 'c']),
            CharacterClasses::Literal("abc".to_string()),
        ];

        assert_equality_test(pattern_str, expected_pattern);
    }
}
