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

impl PatternParser {
    pub fn new(pattern: &str) -> Self {
        PatternParser::from(pattern)
    }

    /// Takes a input string and matches it with the parsed pattern. Return `true` if pattern matches, `false` otherwise
    pub fn match_input_with_pattern(&self, input: &str) -> bool {
        let pattern_vec = &self.0;
        let mut input_peekable = input.chars().peekable();
        let mut result = false;

        while input_peekable.peek().is_some() {
            let mut match_pattern_vec = pattern_vec.iter().map(|x| (false, x)).collect::<Vec<_>>();

            for (matched, pattern) in match_pattern_vec.iter_mut() {
                let char = input_peekable.peek().expect("This should work properly");

                let (match_result_true, iter_step) = match pattern.match_char_with_pattern(*char) {
                    MatchResultType::CharMatch(match_result_true) => (match_result_true, 1),
                    MatchResultType::LiteralMatch(literal) => {
                        let char_position = input.chars().position(|x| x == *char).unwrap(); // the character is definetely in the input

                        let literal_len = literal.len();

                        // get input ref from char_position to char_position + literal_len
                        // Get a slice of the input string from char_position to char_position + literal_len
                        let input_slice: String =
                            input.chars().skip(char_position).take(literal_len).collect();

                        (literal.eq(&input_slice), literal_len)
                    },
                };

                if match_result_true {
                    *matched = true;
                    for _ in 0..iter_step {
                        input_peekable.next();
                    }
                    continue;
                } else {
                    // the character didn't match, so we will break the string and continue with the next first match
                    break;
                }
            }

            if match_pattern_vec.iter().all(|x| x.0) {
                result = true;
                break;
            }
        }
        result
    }
}
pub enum MatchResultType {
    CharMatch(bool),
    LiteralMatch(String),
}

impl CharacterClasses {
    fn match_char_with_pattern(&self, input: char) -> MatchResultType {
        match self {
            CharacterClasses::Digits => {
                MatchResultType::CharMatch(CharacterClasses::is_digit(input))
            },
            CharacterClasses::Characters => {
                MatchResultType::CharMatch(CharacterClasses::is_character(input))
            },
            CharacterClasses::WhiteSpace => {
                MatchResultType::CharMatch(CharacterClasses::is_whitespace(input))
            },
            CharacterClasses::PositiveMatch(positive_match_vec) => MatchResultType::CharMatch(
                CharacterClasses::is_positive_matched(input, positive_match_vec),
            ),
            CharacterClasses::NegativeMatch(negative_match_vec) => MatchResultType::CharMatch(
                CharacterClasses::is_negative_matched(input, negative_match_vec),
            ),
            CharacterClasses::Literal(literal_match) => {
                MatchResultType::LiteralMatch(literal_match.to_string())
            },
        }
    }

    fn is_whitespace(input: char) -> bool {
        input.eq(&' ')
    }

    #[inline(always)]
    fn is_digit(input: char) -> bool {
        input.is_ascii_digit()
    }

    #[inline(always)]
    fn is_character(input: char) -> bool {
        input.is_alphanumeric()
    }

    #[inline(always)]
    fn is_positive_matched(input: char, pattern: &Vec<char>) -> bool {
        pattern.contains(&input)
    }

    #[inline(always)]
    fn is_negative_matched(input: char, pattern: &Vec<char>) -> bool {
        !pattern.contains(&input)
    }
}

#[cfg(test)]
pub mod pattern_parser_tests {
    use super::*;
    use crate::helpers::is_debug_mode;

    pub fn assert_equality_test(pattern_str: &str, expected_pattern: Vec<CharacterClasses>) {
        let parsed_pattern = PatternParser::from(pattern_str).0;

        if is_debug_mode() {
            println!(
                "Pattern string: {pattern_str}\nParsed: {parsed_pattern:#?}\nExpected: \
                 {expected_pattern:#?}"
            );
        }

        assert_eq!(
            expected_pattern, parsed_pattern,
            "The pattern didn't parse into expected pattern: {pattern_str}"
        );
    }

    pub fn assert_pattern_matches(pattern: &str, input: &str) -> bool {
        let parsed_pattern = PatternParser::from(pattern);

        parsed_pattern.match_input_with_pattern(input)
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

    #[test]
    pub fn test_correct_patter_matching_for_input() {
        assert!(assert_pattern_matches("\\d", "1"), "Expected the pattern to match");
    }
}
