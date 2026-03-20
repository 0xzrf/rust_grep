#[derive(Debug, PartialEq, Clone)]
pub enum CharacterClasses {
    /// Digit set of `0-9`
    Digits,
    /// Character set of `a-z`, `A-Z`, `0-9` and `_`
    Characters,
    /// Positive matches tell what should be there. e.g. `[ab]` would match in the positioni only if it has `a or `b``
    PositiveMatch(Vec<char>),
    /// Negative matches tell what should't be there in a postion. e.g. `[^abc]` doesn't match the position which has either `a`, `b` or `c`
    NegativeMatch(Vec<char>),
    /// Matches literal value without change. Note: This is case sensitive
    Literal(String),
    /// Anchor defines the start of a line. e.g., `^log` will match `log`, logs`,`log124`, while not match with `slog`, `blog`.
    Anchor(Vec<CharacterClasses>),
    /// Line anchor defines the end of a line. e.g. dog$ will match `I have a dog`.
    LineAnchor(Vec<CharacterClasses>),
    ImmAnchor(Vec<CharacterClasses>),
    PositiveQuantifier(Box<CharacterClasses>),
    /// Whitespecee
    WhiteSpace,
}

#[derive(Debug)]
pub struct PatternParser(Vec<CharacterClasses>);

impl From<&str> for PatternParser {
    fn from(pattern: &str) -> Self {
        let mut pattern_peek_iterator = pattern.char_indices().peekable();
        let mut is_anchor = false;
        let mut is_line_anchor = false;
        let mut pattern_vec: Vec<CharacterClasses> = vec![];


        if pattern.starts_with("^") {
            let mut peek_anchor_iter = pattern_peek_iterator.clone();
            let mut anchor_char_vec = vec![];
            peek_anchor_iter.next(); // skip `^`

            while let Some((_, char)) = peek_anchor_iter.next_if(|(_, x)| x.ne(&'$')) {
                anchor_char_vec.push(char);
            }

            let anchor_vec_string = anchor_char_vec.into_iter().collect::<String>();

            let pattern_parser = PatternParser::from(anchor_vec_string.as_str()).0;

            pattern_vec.push(CharacterClasses::Anchor(pattern_parser));
            is_anchor = true;
        }

        if pattern.ends_with("$") {
            let mut peek_line_anchor_iter = pattern_peek_iterator.clone();
            let mut line_anchor_char_vec = vec![];

            // skip the first value if it is also an anchor
            if let Some((_, y)) = peek_line_anchor_iter.peek()
                && y == &'^'
            {
                peek_line_anchor_iter.next();
            }

            while let Some((_, char)) = peek_line_anchor_iter.next_if(|(_, x)| x.ne(&'$')) {
                line_anchor_char_vec.push(char);
            }
            let line_vec_string = line_anchor_char_vec.into_iter().collect::<String>();

            let pattern_parser = PatternParser::from(line_vec_string.as_str()).0;

            pattern_vec.push(CharacterClasses::LineAnchor(pattern_parser));
            is_line_anchor = true;
        }

        // when it's both anchor and line anchor, the string has to just consist of the provided pattern inside the `^` and `$`
        if is_anchor && is_line_anchor {
            let pattern_immutable_anchor_peek_iter = pattern_peek_iterator.clone();
            let mut immutable_anchor_char_vec = vec![];

            for (_, char) in pattern_immutable_anchor_peek_iter {
                if char.eq(&'^') || char.eq(&'$') {
                    continue;
                }

                immutable_anchor_char_vec.push(char);
            }

            let pattern_parser = PatternParser::from(
                immutable_anchor_char_vec.into_iter().collect::<String>().as_str(),
            )
            .0;

            return PatternParser(vec![CharacterClasses::ImmAnchor(pattern_parser)]); // early return just the Immutable anchor, since it lies as an excpeption
        } else if is_anchor || is_line_anchor {
            return PatternParser(pattern_vec);
        }

        while pattern_peek_iterator.peek().is_some() {
            let mut skip_next = true;
            match pattern_peek_iterator.peek() {
                Some((_, x)) if x == &'\\' => {
                    pattern_peek_iterator.next();
                    if let Some((_, char)) = pattern_peek_iterator.peek()
                        && char == &'d'
                    {
                        pattern_vec.push(CharacterClasses::Digits)
                    }
                    if let Some((_, char)) = pattern_peek_iterator.peek()
                        && char == &'w'
                    {
                        pattern_vec.push(CharacterClasses::Characters)
                    };
                },
                Some((ix, char)) if char == &'+' => {
                    let last_pattern = pattern_vec.pop().unwrap();
                    println!("last_pattern: {last_pattern:#?}");

                    if let CharacterClasses::Literal(literal) = &last_pattern
                        && literal.len() > 1
                    {
                        let literal_vec = literal.chars().collect::<Vec<char>>();
                        let (remaining, rep_char) = literal_vec.split_at(literal.len() - 1);

                        pattern_vec
                            .push(CharacterClasses::Literal(remaining.iter().collect::<String>()));
                        pattern_vec.push(CharacterClasses::PositiveQuantifier(Box::new(
                            CharacterClasses::Literal(rep_char.iter().collect::<String>()),
                        )));
                    } else {
                        pattern_vec
                            .push(CharacterClasses::PositiveQuantifier(Box::new(last_pattern)));
                    }
                },
                Some((_, y)) if y == &'[' => {
                    pattern_peek_iterator.next();
                    let mut is_neg = false;
                    if let Some((_, char)) = pattern_peek_iterator.peek()
                        && char == &'^'
                    {
                        is_neg = true;
                        pattern_peek_iterator.next(); // skip `^` and jump to the next value, to start putting it in the Vec<char>
                    }
                    // this is a bracket iterrator, so get all the unique values that're there till `]` occurs
                    let mut find_chars_vec: Vec<char> = vec![];

                    // stops when `]` is encountered
                    while let Some((_, unique_char)) =
                        pattern_peek_iterator.next_if(|&(_, x)| x != ']')
                    {
                        find_chars_vec.push(unique_char);
                    }

                    if is_neg {
                        pattern_vec.push(CharacterClasses::NegativeMatch(find_chars_vec));
                    } else {
                        pattern_vec.push(CharacterClasses::PositiveMatch(find_chars_vec));
                    }
                },
                Some((_, space)) if space == &' ' => {
                    pattern_vec.push(CharacterClasses::WhiteSpace);
                },
                Some((_, start_anchor)) if start_anchor == &'^' => {
                    pattern_peek_iterator.next(); // skip the `^` character
                    let mut start_anchor_char_vec: Vec<char> = vec![];

                    for (_, char) in &mut pattern_peek_iterator {
                        start_anchor_char_vec.push(char);
                    }
                    let collected_string = start_anchor_char_vec.into_iter().collect::<String>();
                    let pattern_parser = PatternParser::from(collected_string.as_str()).0;

                    pattern_vec.push(CharacterClasses::Anchor(pattern_parser));
                    skip_next = false;
                },
                Some(_literal) => {
                    let mut literal_char_vec: Vec<char> = vec![];

                    while let Some((ix, char)) = pattern_peek_iterator.next_if(|(_, char)| {
                        char.ne(&'\\') && char.ne(&'[') && char.ne(&' ') && char.ne(&'+')
                    }) {
                        literal_char_vec.push(char);
                    }

                    let literal_string = literal_char_vec.into_iter().collect::<String>();


                    pattern_vec.push(CharacterClasses::Literal(literal_string));


                    skip_next = false;
                },
                None => {},
            }
            if skip_next {
                pattern_peek_iterator.next();
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
        let mut input_peekable = input.char_indices().peekable();
        let mut result = false;

        while input_peekable.peek().is_some() {
            let mut match_pattern_vec = pattern_vec.iter().map(|x| (false, x)).collect::<Vec<_>>();

            for (matched, pattern) in match_pattern_vec.iter_mut() {
                let Some((position_counter, char)) = input_peekable.peek() else { break };

                let (match_result_true, iter_step) = match pattern.match_char_with_pattern(*char) {
                    MatchResultType::Char(match_result_true) => (match_result_true, 1),
                    MatchResultType::Literal(literal) => {
                        let literal_len = literal.len();

                        // get input ref from char_position to char_position + literal_len
                        // Get a slice of the input string from char_position to char_position + literal_len
                        let input_slice: String =
                            input.chars().skip(*position_counter).take(literal_len).collect();

                        (literal.eq(&input_slice), literal_len)
                    },
                    MatchResultType::Anchor(anchor_match_vec) => {
                        // we will have to assume that position_counter == 0
                        let mut anchor_input_peekable_iter = input.char_indices().peekable();

                        for pattern in &anchor_match_vec {
                            let Some((position_counter, char)) = anchor_input_peekable_iter.peek()
                            else {
                                break;
                            };
                            let (matched_result, to_skip) = match pattern
                                .match_char_with_pattern(*char)
                            {
                                MatchResultType::Char(match_result_type) => (match_result_type, 1),
                                MatchResultType::Literal(literal) => {
                                    let literal_len = literal.len();

                                    // get input ref from char_position to char_position + literal_len
                                    // Get a slice of the input string from char_position to char_position + literal_len
                                    let input_slice: String = input
                                        .chars()
                                        .skip(*position_counter)
                                        .take(literal_len)
                                        .collect();

                                    (literal.eq(&input_slice), literal_len)
                                },
                                _ => {
                                    unreachable!()
                                },
                            };
                            if !matched_result {
                                return false;
                            } else {
                                for _ in 0..to_skip {
                                    input_peekable.next();
                                }
                            }
                        }

                        // if the loop ran without anything being false, then return true
                        return true;
                    },
                    MatchResultType::LineAnchor(mut pattern_match_reverse) => {
                        pattern_match_reverse.reverse();
                        let mut input_pattern_reverse = input.char_indices().rev().peekable();

                        for pattern in &pattern_match_reverse {
                            let Some((position_counter, char)) = input_pattern_reverse.peek()
                            else {
                                break;
                            };

                            let (matched_result, to_skip): (bool, usize) =
                                match pattern.match_char_with_pattern(*char) {
                                    MatchResultType::Char(match_result) => (match_result, 1),
                                    MatchResultType::Literal(literal) => {
                                        let Some(start_ix) =
                                            position_counter.checked_sub(literal.len() - 1)
                                        else {
                                            return false; // in case that the literal is bigger then the position counter, it's a case where the input isn't big enough for the pattern, hence it doesn't match
                                        };

                                        let input_slice = input
                                            .chars()
                                            .skip(start_ix)
                                            .take(literal.len())
                                            .collect::<String>();


                                        (literal.eq(&input_slice), literal.len())
                                    },
                                    _ => {
                                        // there won't be an internal anchor or line anchor inside a line anchor
                                        unreachable!()
                                    },
                                };

                            if !matched_result {
                                return false; // do an early return if it fails to match by any chance
                            } else {
                                for _ in 0..to_skip {
                                    input_pattern_reverse.next();
                                }
                            }
                        }

                        return true;
                    },
                    MatchResultType::ImmAnchor(imm_anchor) => {
                        // early return if the length of the value isn't == expected
                        let mut expected_len = 0u64;
                        for pattern in &imm_anchor {
                            match pattern {
                                CharacterClasses::Digits
                                | CharacterClasses::WhiteSpace
                                | CharacterClasses::Characters
                                | CharacterClasses::NegativeMatch(_)
                                | CharacterClasses::PositiveMatch(_) => expected_len += 1,
                                CharacterClasses::Literal(literal) => {
                                    expected_len += literal.len() as u64
                                },
                                _ => {
                                    unreachable!()
                                },
                            }
                        }

                        if input.len() != expected_len as usize {
                            return false; // an early return is completely valid if the input isn't exactly the size of the expected pattern
                        }

                        // we will have to assume that position_counter == 0
                        let mut imm_anchor_input_peekable_iter = input.char_indices().peekable();

                        for pattern in &imm_anchor {
                            let Some((position_counter, char)) =
                                imm_anchor_input_peekable_iter.peek()
                            else {
                                break;
                            };
                            let (matched_result, to_skip) = match pattern
                                .match_char_with_pattern(*char)
                            {
                                MatchResultType::Char(match_result_type) => (match_result_type, 1),
                                MatchResultType::Literal(literal) => {
                                    let literal_len = literal.len();

                                    // get input ref from char_position to char_position + literal_len
                                    // Get a slice of the input string from char_position to char_position + literal_len
                                    let input_slice: String = input
                                        .chars()
                                        .skip(*position_counter)
                                        .take(literal_len)
                                        .collect();

                                    (literal.eq(&input_slice), literal_len)
                                },
                                _ => {
                                    unreachable!()
                                },
                            };
                            if !matched_result {
                                return false;
                            } else {
                                for _ in 0..to_skip {
                                    input_peekable.next();
                                }
                            }
                        }

                        // if the loop ran without anything being false, then return true
                        return true;
                    },
                    MatchResultType::Qualifier(QualifierType::Positive(target_pattern)) => {
                        let mut pattern_count = 0u64;
                        while input_peekable
                            .next_if(|(_ix, char)| {
                                match target_pattern.as_ref().match_char_with_pattern(*char) {
                                    MatchResultType::Char(match_result) => {
                                        if match_result {
                                            pattern_count += 1;
                                            true
                                        } else {
                                            false
                                        }
                                    },
                                    MatchResultType::Literal(target_char) => {
                                        if target_char.eq(&char.to_string()) {
                                            pattern_count += 1;
                                            true
                                        } else {
                                            false
                                        }
                                    },
                                    _ => {
                                        unreachable!() // positive qualifier won't have a anchors
                                    },
                                }
                            })
                            .is_some()
                        {}

                        (pattern_count >= 1, 0)
                    },
                    MatchResultType::Qualifier(QualifierType::Lazy(char)) => {
                        todo!()
                    },
                    MatchResultType::Qualifier(QualifierType::Greedy(char)) => {
                        todo!()
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
                    input_peekable.next();

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
    Char(bool),
    Literal(String),
    Anchor(Vec<CharacterClasses>),
    LineAnchor(Vec<CharacterClasses>),
    ImmAnchor(Vec<CharacterClasses>),
    Qualifier(QualifierType<CharacterClasses>),
}

pub enum QualifierType<T> {
    Positive(Box<T>),
    Lazy(T),
    Greedy(T),
}

impl CharacterClasses {
    fn match_char_with_pattern(&self, input: char) -> MatchResultType {
        match self {
            CharacterClasses::Digits => MatchResultType::Char(CharacterClasses::is_digit(input)),
            CharacterClasses::Characters => {
                MatchResultType::Char(CharacterClasses::is_character(input))
            },
            CharacterClasses::WhiteSpace => {
                MatchResultType::Char(CharacterClasses::is_whitespace(input))
            },
            CharacterClasses::PositiveMatch(positive_match_vec) => MatchResultType::Char(
                CharacterClasses::is_positive_matched(input, positive_match_vec),
            ),
            CharacterClasses::NegativeMatch(negative_match_vec) => MatchResultType::Char(
                CharacterClasses::is_negative_matched(input, negative_match_vec),
            ),
            CharacterClasses::Literal(literal_match) => {
                MatchResultType::Literal(literal_match.to_string())
            },
            CharacterClasses::Anchor(start_anchor) => MatchResultType::Anchor(start_anchor.clone()),
            CharacterClasses::LineAnchor(line_anchor) => {
                MatchResultType::LineAnchor(line_anchor.clone())
            },
            CharacterClasses::ImmAnchor(imm_anchor) => {
                MatchResultType::ImmAnchor(imm_anchor.clone())
            },
            CharacterClasses::PositiveQuantifier(c) => {
                MatchResultType::Qualifier(QualifierType::Positive(c.clone()))
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
        input.is_alphanumeric() || input.eq(&'_')
    }

    #[inline(always)]
    fn is_positive_matched(input: char, pattern: &[char]) -> bool {
        pattern.contains(&input)
    }

    #[inline(always)]
    fn is_negative_matched(input: char, pattern: &[char]) -> bool {
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

        println!("parsed_pattern: {parsed_pattern:#?}");

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

        let pattern_str = "^str";
        let expected_pattern =
            vec![CharacterClasses::Anchor(vec![CharacterClasses::Literal("str".to_string())])];

        assert_equality_test(pattern_str, expected_pattern);
        let pattern_str = "^\\d\\d\\d";
        let expected_pattern = vec![CharacterClasses::Anchor(vec![
            CharacterClasses::Digits,
            CharacterClasses::Digits,
            CharacterClasses::Digits,
        ])];

        assert_equality_test(pattern_str, expected_pattern);

        let pattern_str = "^\\d\\w \\d";
        let expected_pattern = vec![CharacterClasses::Anchor(vec![
            CharacterClasses::Digits,
            CharacterClasses::Characters,
            CharacterClasses::WhiteSpace,
            CharacterClasses::Digits,
        ])];

        assert_equality_test(pattern_str, expected_pattern);

        let pattern_str = "end$";
        let expected_pattern =
            vec![CharacterClasses::LineAnchor(vec![CharacterClasses::Literal("end".to_string())])];

        assert_equality_test(pattern_str, expected_pattern);

        let pattern_str = "\\d\\d\\w$";
        let expected_pattern = vec![CharacterClasses::LineAnchor(vec![
            CharacterClasses::Digits,
            CharacterClasses::Digits,
            CharacterClasses::Characters,
        ])];

        assert_equality_test(pattern_str, expected_pattern);

        let pattern_str = "\\d\\d \\w$";
        let expected_pattern = vec![CharacterClasses::LineAnchor(vec![
            CharacterClasses::Digits,
            CharacterClasses::Digits,
            CharacterClasses::WhiteSpace,
            CharacterClasses::Characters,
        ])];

        assert_equality_test(pattern_str, expected_pattern);

        let pattern_str = "^orange$";
        let expected_pattern = vec![CharacterClasses::ImmAnchor(vec![CharacterClasses::Literal(
            "orange".to_string(),
        )])];

        assert_equality_test(pattern_str, expected_pattern);

        let pattern_str = "orange+";
        let expected_pattern = vec![
            CharacterClasses::Literal("orang".to_string()),
            CharacterClasses::PositiveQuantifier(Box::new(CharacterClasses::Literal(
                "e".to_string(),
            ))),
        ];

        assert_equality_test(pattern_str, expected_pattern);


        let pattern_str = "or+ange";
        let expected_pattern = vec![
            CharacterClasses::Literal("o".to_string()),
            CharacterClasses::PositiveQuantifier(Box::new(CharacterClasses::Literal(
                "r".to_string(),
            ))),
            CharacterClasses::Literal("ange".to_string()),
        ];

        assert_equality_test(pattern_str, expected_pattern);

        let pattern_str = "\\d+ days";
        let expected_pattern = vec![
            CharacterClasses::PositiveQuantifier(Box::new(CharacterClasses::Digits)),
            CharacterClasses::WhiteSpace,
            CharacterClasses::Literal("days".to_string()),
        ];

        assert_equality_test(pattern_str, expected_pattern);
    }

    #[test]
    pub fn test_correct_patter_matching_for_input() {
        // BASIC TESTS
        assert!(assert_pattern_matches("\\d", "1"), "Expected the pattern to match");
        assert!(!assert_pattern_matches("\\d", "w"), "Expected the pattern to not match");
        assert!(assert_pattern_matches("\\d apple", "1 apple"), "Expected the pattern to match");
        assert!(
            assert_pattern_matches("apples \\d\\d\\d", "I got apples 100 from the store"),
            "Expected the pattern to match"
        );
        assert!(
            !assert_pattern_matches("\\d\\d\\d apples", "I got 1 apples from the store"),
            "Expected the pattern to not match"
        );
        assert!(
            !assert_pattern_matches("\\d apples", "1 oranges"),
            "Expected the pattern to not match"
        );
        assert!(
            assert_pattern_matches("\\d \\w\\w\\ws", "2 cats"),
            "Expected the pattern to match"
        );

        assert!(
            assert_pattern_matches("\\d apple", "sally has 3 apples"),
            "Expected the pattern to match"
        );
        assert!(
            !assert_pattern_matches("\\d \\w\\w\\ws", "sally has 1 dog"),
            "Expected the pattern to not match"
        );

        // ANCHOR TESTS
        assert!(assert_pattern_matches("^log", "log"), "Expected the pattern to match");
        assert!(assert_pattern_matches("^\\d", "1"), "Expected the pattern to match");
        assert!(assert_pattern_matches("^\\d\\d\\d", "100s"), "Expected the pattern to match");
        assert!(
            !assert_pattern_matches("^\\d\\d\\d", "wwe"),
            "Expected the pattern to not match"
        );
        assert!(!assert_pattern_matches("^log", "slog"), "Expected the pattern to not match");

        // LINE ANCHOR TESTS
        assert!(assert_pattern_matches("log$", "this pattern will match this coz it has log"));
        assert!(assert_pattern_matches("\\d dog has \\w\\w\\ws", "my 1 dog has logs"));
        assert!(assert_pattern_matches("\\w\\w\\ws", "I have 3 cats"));
        assert!(!assert_pattern_matches("\\w\\w\\w\\w is a joke", "he is a joke"));
        assert!(assert_pattern_matches("^orange$", "orange"));
        assert!(!assert_pattern_matches("^respberry$", "respberry_respberry"));
        assert!(assert_pattern_matches("orange+", "orange"));
        assert!(assert_pattern_matches("orange+", "orangeeeeee"));
        assert!(assert_pattern_matches("or+ange", "orrrrrrrange"));
    }
}
