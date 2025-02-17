//! THe tokenizer module provides support for translating the literal contents
//! of a Scott Adams adventure file in the ScottFree (TRS-80) format into a
//! sequence of tokens.
//!
//! Such a file consists of a sequence of ASCII-formatted integers (possibly
//! with surrounding whitespace) and quote-delimited strings (possibly with
//! internal newlines).
//!
//! We don't pay the slightest bit of attention to Unicode or processing the
//! data as runes, since this file format is from the 8-bit days.

use std::collections::VecDeque;
use std::fmt::{Display, Error, Formatter};

/// A Location identifies line number and column within the original game file.
#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

/// There are only two kinds of token, Int and Str.
#[derive(Debug)]
pub enum Token {
    Int(i32, Location),
    Str(String, Location),
}

/// A Stream contains a fully parsed sequence of tokens and a current-position
/// marker.
pub struct Stream {
    tokens: VecDeque<Token>,
}

/// These states are used by the finite state machine in `new` for parsing the
/// input data.  The individual states are documented inline.
#[derive(Debug)]
pub enum State {
    Init,
    Sign,
    Num,
    Quote,
    Escape,
}

impl Stream {
    /// new initializes a new Stream from the given game data.  Because the
    /// game files are small and we never read them partially, we do all of
    /// the parsing up front.
    pub fn new(data: Vec<u8>) -> Result<Stream, TokenError> {
        let mut tokens = VecDeque::new();
        let mut state = State::Init;
        let mut acc = String::new();

        let mut current_loc = Location{line: 1, col: 1};
        let mut token_loc = Location{line: 1, col: 1};

        for offset in 0..data.len() {
            let ch = *data.get(offset).unwrap() as char;
            if ch == '\n' {
                current_loc.line += 1;
                current_loc.col = 1;
            } else {
                current_loc.col += 1;
            }

            match state {
                // Init state: Not currently reading any token.
                State::Init => {
                    if ch.is_ascii_whitespace() {
                        // pass
                    } else if ch == '-' {
                        token_loc = current_loc;
                        acc.push(ch);
                        state = State::Sign;
                    } else if ch.is_ascii_digit() {
                        token_loc = current_loc;
                        acc.push(ch);
                        state = State::Num;
                    } else if ch == '"' {
                        token_loc = current_loc;
                        state = State::Quote;
                    } else {
                        return Err(TokenError { loc: current_loc, msg: format!("Unexpected character '{}'", ch) });
                    }
                }

                // Sign state: Read the initial '-' of a negative integer.
                State::Sign => {
                    if ch.is_ascii_digit() {
                        acc.push(ch);
                        state = State::Num;
                    } else {
                        return Err(TokenError { loc: current_loc, msg: format!("Unexpected character '{}' in integer", ch) });
                    }
                }

                // Num state: Now reading an integer.
                State::Num => {
                    if ch.is_ascii_whitespace() {
                        match acc.parse::<i32>() {
                            Ok(val) => tokens.push_back(Token::Int(val, token_loc.clone())),
                            Err(_) => return Err(TokenError { loc: current_loc, msg: "Malformed integer".to_string() }),
                        }
                        acc.clear();
                        state = State::Init;
                    } else if ch.is_ascii_digit() {
                        acc.push(ch);
                    } else {
                        return Err(TokenError { loc: current_loc, msg: format!("Unexpected character '{}' in integer", ch) });
                    }
                }

                // Quote state: Read the initial '"' of a string.
                State::Quote => {
                    if ch == '\\' {
                        state = State::Escape;
                    } else if ch == '"' {
                        tokens.push_back(Token::Str(acc.clone(), token_loc.clone()));
                        acc.clear();
                        state = State::Init;
                    } else {
                        acc.push(ch);
                    }
                }

                // Escape state: Read the next character in a string unconditionally.
                State::Escape => {
                    acc.push(ch);
                    state = State::Quote;
                }
            }
        }
        Ok(Stream { tokens })
    }

    /// Checks if we're at the end of the stream.
    pub fn done(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Returns the next integer in the stream.
    pub fn next_int(&mut self) -> Result<i32, TokenError> {
        println!("next_int");
        match self.tokens.pop_front() {
            Some(Token::Int(val, _)) => Ok(val),
            Some(Token::Str(_, loc)) => Err(TokenError{ loc, msg: "Expected an integer, found a string".to_string() }),
            None => Err(TokenError{ loc: Location{line: 0, col: 0}, msg: "Unexpected end of stream".to_string() }),
        }
    }

    /// Returns the next string in the stream.
    pub fn next_str(&mut self) -> Result<String, TokenError> {
        println!("next_str");
        match self.tokens.pop_front() {
            Some(Token::Str(val, _)) => Ok(val),
            Some(Token::Int(_, loc)) => Err(TokenError{ loc, msg: "Expected a string, found an integer".to_string() }),
            None => Err(TokenError{ loc: Location{line: 0, col: 0}, msg: "Unexpected end of stream".to_string() }),
        }
    }

    /// Returns the next token.
    pub fn next_token(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }
}

/// Represents an error encountered during tokenization.
pub struct TokenError {
    loc: Location,
    msg: String,
}

impl Display for TokenError {
    /// Makes a tokenization error human-readable.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}:{}: {}", self.loc.line, self.loc.col, self.msg)
    }
}
