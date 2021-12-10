use either::Either;

fn main() {
    let test = include_str!("../../../input/day_10_test.txt")
        .lines()
        .collect::<Vec<_>>();
    let input = include_str!("../../../input/day_10_input.txt")
        .lines()
        .collect::<Vec<_>>();

    assert_eq!(26397, dbg!(part_1(&test)));
    dbg!(part_1(&input));
    assert_eq!(288957, dbg!(part_2(&test)));
    dbg!(part_2(&input));
}

fn part_1(lines: &[&str]) -> usize {
    let mut count = 0;
    for line in lines {
        if let Some(token) = get_corrupt_token(line) {
            count += token.found.invalid_close_score();
        }
    }
    count
}

fn part_2(lines: &[&str]) -> usize {
    let mut scores = Vec::new();
    for line in lines {
        if let Either::Right(stack) = get_corrupt_token_or_missing_stack(line) {
            let mut line_count = 0;
            let stack: Vec<Token> = stack.into_iter().rev().map(|idx| TOKEN_MAP[idx]).collect();
            for token in &stack {
                line_count *= 5;
                line_count += token.incomplete_line_score();
            }
            scores.push(line_count);
        }
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Token {
    /// ( .. )
    Parenthesis,
    /// [ .. ]
    Bracket,
    /// { .. }
    Brace,
    /// < .. >
    Gt,
}
impl Token {
    fn invalid_close_score(&self) -> usize {
        match self {
            Self::Parenthesis => 3,
            Self::Bracket => 57,
            Self::Brace => 1197,
            Self::Gt => 25137,
        }
    }
    fn incomplete_line_score(&self) -> usize {
        match self {
            Self::Parenthesis => 1,
            Self::Bracket => 2,
            Self::Brace => 3,
            Self::Gt => 4,
        }
    }
}
const OPEN_TOKENS: &[char] = &['(', '[', '{', '<'];
const CLOSE_TOKENS: &[char] = &[')', ']', '}', '>'];
const TOKEN_MAP: &[Token] = &[Token::Parenthesis, Token::Bracket, Token::Brace, Token::Gt];

#[derive(PartialEq, Eq, Debug)]
struct CorruptTokenResponse {
    expected: Token,
    found: Token,
}
fn get_corrupt_token_or_missing_stack(line: &str) -> Either<CorruptTokenResponse, Vec<usize>> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if let Some(idx) = OPEN_TOKENS.iter().position(|t| t == &c) {
            stack.push(idx);
        } else if let Some(idx) = CLOSE_TOKENS.iter().position(|t| t == &c) {
            if let Some(last) = stack.pop() {
                if idx != last {
                    return Either::Left(CorruptTokenResponse {
                        expected: TOKEN_MAP[last],
                        found: TOKEN_MAP[idx],
                    });
                }
            }
        } else {
            panic!("Unknown char: {:?}", c);
        }
    }
    Either::Right(stack)
}

fn get_corrupt_token(line: &str) -> Option<CorruptTokenResponse> {
    get_corrupt_token_or_missing_stack(line).left()
}

#[test]
fn test_corrupt_tokens() {
    assert_eq!(
        get_corrupt_token("{([(<{}[<>[]}>{[]{[(<()>"),
        Some(CorruptTokenResponse {
            expected: Token::Bracket,
            found: Token::Brace
        })
    );
    assert_eq!(
        get_corrupt_token("[[<[([]))<([[{}[[()]]]"),
        Some(CorruptTokenResponse {
            expected: Token::Bracket,
            found: Token::Parenthesis
        })
    );
    assert_eq!(
        get_corrupt_token("[{[{({}]{}}([{[{{{}}([]"),
        Some(CorruptTokenResponse {
            expected: Token::Parenthesis,
            found: Token::Bracket
        })
    );
    assert_eq!(
        get_corrupt_token("[<(<(<(<{}))><([]([]()"),
        Some(CorruptTokenResponse {
            expected: Token::Gt,
            found: Token::Parenthesis
        })
    );
    assert_eq!(
        get_corrupt_token("<{([([[(<>()){}]>(<<{{"),
        Some(CorruptTokenResponse {
            expected: Token::Bracket,
            found: Token::Gt
        })
    );
}
