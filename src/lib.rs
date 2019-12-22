#![allow(incomplete_features)]
#![allow(dead_code)]
#![feature(const_generics)]
#![feature(stmt_expr_attributes)]
// #![feature(or_patterns)]

#[test]
mod tests;

#[derive(Debug, PartialEq)]
pub struct TokenizerError {
    pub error: ErrorType,
    pub index: usize,
}

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    /// An escape was found but not followed by any characters
    TrailingEscape,
    /// An opening quotation mark was not followed by a matching close
    UnbalancedQuotes,
    /// Unable to parse input as valid UTF-8 string
    Utf8Error,
}

#[derive(PartialEq)]
pub enum Until {
    Quote,
    DoubleQuote,
}

#[derive(PartialEq)]
enum Skip {
    None,
    Any,
    Until(char),
}

type TokenStart = usize;
type QuoteStart = usize;
enum State {
    None,
    /// Within a token
    TokenStarted,
    /// Within a double quote
    DoubleQuoted(QuoteStart),
    /// Within a single quote
    SingleQuoted(QuoteStart),
    // /// After a single-character escape
    // Escaped,
}

pub enum TokenType<'a> {
    Allocated(String),
    Reference(&'a str),
}

// pub fn tokenize<'a>(s: &'a str) -> Result<Vec<TokenType<'a>>, TokenizerError> {
pub fn tokenize<'a>(s: &'a str) -> Result<Vec<String>, TokenizerError> {
    let mut tokens = Vec::new();

    let mut range_start: Option<usize> = None;
    let mut token_ranges: Vec<&'a str> = Vec::new();
    let mut state = State::None;
    let mut iter = s.char_indices().peekable();

    while let Some((i, c)) = iter.next() {
        let mut maybe_end_range = #[inline(always)]
        || {
            if let Some(start) = range_start {
                token_ranges.push(s.get(start..i).unwrap());
                range_start = None;
            }
        };

        match state {
            State::None => match c {
                '\\' => {
                    state = State::TokenStarted;
                    match iter.peek() {
                        Some((n_i, n_c)) => match n_c {
                            '\\' | '"' | '\'' => {
                                range_start = Some(*n_i);
                                iter.next();
                                continue;
                            }
                            _ => {}
                        },
                        None => {
                            return Err(TokenizerError {
                                error: ErrorType::TrailingEscape,
                                index: i,
                            })
                        }
                    }
                }
                '\'' => state = State::SingleQuoted(i),
                '"' => state = State::DoubleQuoted(i),
                ' ' | '\t' | '\r' | '\n' => {
                    if !token_ranges.is_empty() {
                        tokens.push(token_ranges.concat());
                        token_ranges.clear();
                    }
                }
                _ => {
                    state = State::TokenStarted;
                    range_start = Some(i);
                }
            },
            State::DoubleQuoted(_) => {
                match c {
                    '"' => {
                        state = State::None;
                        maybe_end_range();
                        continue;
                    }
                    '\\' => match iter.peek() {
                        Some((n_i, '\\')) | Some((n_i, '"')) => {
                            maybe_end_range();
                            range_start = Some(*n_i);
                            iter.next();
                        }
                        _ => {}
                    },
                    _ => {}
                }
                range_start = range_start.or(Some(i));
            }
            State::SingleQuoted(_) => {
                match c {
                    '\'' => {
                        state = State::None;
                        maybe_end_range();
                        continue;
                    }
                    '\\' => match iter.peek() {
                        Some((n_i, n_c)) => match n_c {
                            '\\' | '\'' => {
                                maybe_end_range();
                                range_start = Some(*n_i);
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                }
                range_start = range_start.or(Some(i));
            }
            State::TokenStarted => match c {
                '\\' => match iter.peek() {
                    Some((n_i, n_c)) => match n_c {
                        '\\' | '"' | '\'' => {
                            maybe_end_range();
                            range_start = Some(*n_i);
                            iter.next();
                            continue;
                        }
                        _ => {}
                    },
                    None => {
                        return Err(TokenizerError {
                            error: ErrorType::TrailingEscape,
                            index: i,
                        })
                    }
                },
                ' ' | '\t' | '\r' | '\n' => {
                    maybe_end_range();
                    tokens.push(token_ranges.concat());
                    token_ranges.clear();
                    state = State::None;
                }
                _ => {}
            },
        }
    }

    match state {
        State::DoubleQuoted(q) | State::SingleQuoted(q) => Err(TokenizerError {
            error: ErrorType::UnbalancedQuotes,
            index: q,
        }),
        State::TokenStarted => {
            token_ranges.push(s.get(range_start.unwrap()..).unwrap());
            tokens.push(token_ranges.concat());
            token_ranges.clear();
            Ok(tokens)
        }
        State::None => {
            if token_ranges.len() > 0 {
                tokens.push(token_ranges.concat());
            }
            Ok(tokens)
        }
    }
}
