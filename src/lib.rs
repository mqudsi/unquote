#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]

#[cfg(test)]
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
    /// Invalid escape
    InvalidEscape,
}

type QuoteStart = usize;
#[derive(Clone, Copy, Debug)]
enum State {
    None,
    /// Within a token
    TokenStarted,
    /// Within a double quote
    DoubleQuoted(QuoteStart),
    /// Within a single quote
    SingleQuoted(QuoteStart),
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
            if let Some(start) = range_start.take() {
                token_ranges.push(s.get(start..i).unwrap());
            }
        };

        if c == '\\' {
            let next = match iter.peek() {
                None => {
                    return Err(TokenizerError {
                        error: ErrorType::TrailingEscape,
                        index: i,
                    })
                }
                Some(next) => next,
            };

            let push;
            match (state, next.1) {
                (_, '\\') => push = Some("\\"),
                // Within single quotes, only an escaped backslash or single quote is allowed
                (State::None, '\'') | (State::SingleQuoted(_), '\'') => push = Some("'"),
                // Otherwise, single quotes imply "raw" text, including backslashes
                (State::SingleQuoted(_), _) => push = None,
                (State::None, '\"') | (State::DoubleQuoted(_), '\"') => push = Some("\""),
                // These are valid both within double quotes and in unquoted text
                (_, 't') => push = Some("\t"),
                (_, 'n') => push = Some("\n"),
                (_, 'r') => push = Some("\r"),
                _ => {
                    return Err(TokenizerError {
                        error: ErrorType::InvalidEscape,
                        index: next.0,
                    })
                }
            }

            if let Some(push) = push {
                maybe_end_range();
                tokens.push(push.to_owned());
                iter.next();
                continue;
            }
        }

        match state {
            State::None => match c {
                '\'' => {
                    state = State::SingleQuoted(i);
                    range_start = iter.peek().map(|n| n.0);
                }
                '"' => {
                    state = State::DoubleQuoted(i);
                    range_start = iter.peek().map(|n| n.0);
                }
                ' ' | '\t' | '\r' | '\n' => continue,
                _ => {
                    state = State::TokenStarted;
                    range_start = Some(i);
                }
            },
            State::TokenStarted => match c {
                '\'' => {
                    state = State::SingleQuoted(i);
                    token_ranges.push(s.get(range_start.unwrap()..i).unwrap());
                    range_start = iter.peek().map(|n| n.0);
                }
                '"' => {
                    state = State::DoubleQuoted(i);
                    token_ranges.push(s.get(range_start.unwrap()..i).unwrap());
                    range_start = iter.peek().map(|n| n.0);
                }
                ' ' | '\t' | '\r' | '\n' => {
                    token_ranges.push(s.get(range_start.unwrap()..i).unwrap());
                    tokens.push(token_ranges.concat());
                    token_ranges.clear();
                    state = State::None;
                }
                _ => {}
            },
            State::DoubleQuoted(_) => match c {
                '"' => {
                    state = State::TokenStarted;
                    maybe_end_range();
                    range_start = iter.peek().map(|n| n.0);
                    continue;
                }
                _ => {}
            },
            State::SingleQuoted(_) => match c {
                '\'' => {
                    state = State::TokenStarted;
                    maybe_end_range();
                    range_start = iter.peek().map(|n| n.0);
                    continue;
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
            if let Some(range_start) = range_start {
                token_ranges.push(s.get(range_start..).unwrap());
            }
            tokens.push(token_ranges.concat());
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
