//! This module contains all of the code used to initialize a Game structure
//! from a collection of tokens.

use std::fmt::{Display, Error, Formatter};

use super::*;
use crate::tokenizer::Stream;

/// Initializes a new Game structure from a stream of tokens.
pub fn parse_game(stream: &mut Stream) -> Result<Game, ParseError> {
    let header = match parse_header(stream) {
        Ok(header) => header,
        Err(e) => return Err(e),
    };

    let actions = match parse_actions(stream, header.num_actions) {
        Ok(actions) => actions,
        Err(e) => return Err(e),
    };

    let words = match parse_words(stream, header.num_words) {
        Ok(words) => words,
        Err(e) => return Err(e),
    };

    Ok(Game {
        header,
        actions,
        verbs: words.0,
        nouns: words.1,
    })
}

/// Parses the header of the game file.
fn parse_header(stream: &mut Stream) -> Result<Header, ParseError> {
    Ok(Header {
        unknown0: _read_int(stream)?,
        num_items: _read_int(stream)? + 1, // adjust for option base 0
        num_actions: _read_int(stream)? + 1, // adjust for option base 0
        num_words: _read_int(stream)? + 1, // adjust for option base 0
        num_rooms: _read_int(stream)? + 1, // adjust for option base 0
        max_inventory: _read_int(stream)?,
        starting_room: _read_int(stream)?,
        num_treasures: _read_int(stream)?,
        word_length: _read_int(stream)?,
        light_duration: _read_int(stream)?,
        num_messages: _read_int(stream)? + 1, // adjust for option base 0
        treasure_room: _read_int(stream)?,
    })
}

/// Parses all of the actions from the game file.
fn parse_actions(stream: &mut Stream, num_actions: i32) -> Result<Vec<Action>, ParseError> {
    let mut actions = Vec::new();
    for _ in 0..num_actions {
        let action = parse_action(stream)?;
        actions.push(action);
    }
    Ok(actions)
}

/// Parses a single action from the game file.  Each action has the following form:
/// *   (150 * verb index) + noun index
/// *   5x conditions, expressed as condition type + (20 * value)
/// *   (150 * action0 type) + action1 type
/// *   (150 * action2 type) + action3 type
fn parse_action(stream: &mut Stream) -> Result<Action, ParseError> {
    let num = _read_int(stream)?;
    let verb_index = num / 150;
    let noun_index = num % 150;

    let mut conditions = [(); 5].map(|_| Condition::default());
    for cond in &mut conditions {
        let num = _read_int(stream)?;
        cond.cond_type = (num % 20) as ConditionType;
        cond.value = num / 20;
    }

    let mut actions = [(); 4].map(|_| ActionType::default());
    for i in 0..2 {
        let num = _read_int(stream)?;
        actions[i * 2] = super::ActionType::Generic(num / 150);
        actions[i * 2 + 1] = super::ActionType::Generic(num % 150);
    }

    Ok(Action {
        verb_index,
        noun_index,
        conditions,
        actions,
    })
}

/// Parses all of the words from the game file, which are an interleaved array
/// of strings.  An initial "*" indicates a synonym.
fn parse_words(stream: &mut Stream, num_words: i32) -> Result<(Vec<Word>, Vec<Word>), ParseError> {
    let mut verbs = Vec::new();
    let mut nouns = Vec::new();

    for _ in 0..num_words {
        let verb = _read_word(stream)?;
        verbs.push(verb);
        let noun = _read_word(stream)?;
        nouns.push(noun);
    }
    Ok((verbs, nouns))
}

fn _read_int(stream: &mut Stream) -> Result<i32, ParseError> {
    match stream.next_int() {
        Ok(value) => Ok(value),
        Err(e) => Err(ParseError { msg: e }),
    }
}

fn _read_str(stream: &mut Stream) -> Result<String, ParseError> {
    match stream.next_str() {
        Ok(value) => Ok(value),
        Err(e) => Err(ParseError { msg: e }),
    }
}

fn _read_word(stream: &mut Stream) -> Result<Word, ParseError> {
    let mut word = _read_str(stream)?;
    let mut is_synonym = false;
    if word.starts_with("*") {
        is_synonym = true;
        word = word.strip_prefix("*").unwrap().to_string();
    }
    Ok(Word { word, is_synonym })
}

/// Represents an error encountered during parsing.
pub struct ParseError {
    msg: String,
}

impl Display for ParseError {
    /// Makes a parsing error human-readable.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.msg)
    }
}
