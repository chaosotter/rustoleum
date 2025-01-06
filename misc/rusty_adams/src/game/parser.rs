//! This module contains all of the code used to initialize a Game structure
//! from a collection of tokens.

use std::fmt::{Display, Error, Formatter};

use crate::tokenizer;

/// Initializes a new Game structure from a stream of tokens.
pub fn parse_game(stream: &mut tokenizer::Stream) -> Result<super::Game, ParseError> {
    let header = match parse_header(stream) {
        Ok(header) => header,
        Err(e) => return Err(e),
    };

    let actions = match parse_actions(stream, header.num_actions) {
        Ok(actions) => actions,
        Err(e) => return Err(e),
    };

    Ok(super::Game {
        header: header,
        actions: actions,
    })
}

/// Parses the header of the game file.
fn parse_header(stream: &mut tokenizer::Stream) -> Result<super::Header, ParseError> {
    Ok(super::Header {
        unknown0: _read_int(stream)?,
        num_items: _read_int(stream)? + 1, // adjust for option base 0
        num_actions: _read_int(stream)? + 1, // adjust for option base 0
        num_words: _read_int(stream)?,     // adjust for option base 0
        num_rooms: _read_int(stream)?,     // adjust for option base 0
        max_inventory: _read_int(stream)?,
        starting_room: _read_int(stream)?,
        num_treasures: _read_int(stream)?,
        word_length: _read_int(stream)?,
        light_duration: _read_int(stream)?,
        num_messages: _read_int(stream)?, // adjust for option base 0
        treasure_room: _read_int(stream)?,
    })
}

/// Parses all of the actions from the game file.
fn parse_actions(stream: &mut tokenizer::Stream, num_actions: i32) -> Result<Vec<super::Action>, ParseError> {
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
fn parse_action(stream: &mut tokenizer::Stream) -> Result<super::Action, ParseError> {
    let num = _read_int(stream)?;
    let verb_index = num / 150;
    let noun_index = num % 150;

    let mut conditions = [(); 5].map(|_| super::Condition::default());
    for i in 0..5 {
        let num = _read_int(stream)?;
        conditions[i] = super::Condition {
            cond_type: (num % 20) as super::ConditionType,
            value: num / 20,
        };
    }

    let mut actions = [(); 4].map(|_| super::ActionType::default());
    for i in 0..2 {
        let num = _read_int(stream)?;
        actions[i*2] = super::ActionType::Generic(num / 150);
        actions[i*2 + 1] = super::ActionType::Generic(num % 150);
    }

    Ok(super::Action {
        verb_index,
        noun_index,
        conditions,
        actions: actions,
    })
}

fn _read_int(stream: &mut tokenizer::Stream) -> Result<i32, ParseError> {
    match stream.next_int() {
        Ok(value) => Ok(value),
        Err(e) => Err(ParseError { msg: e }),
    }
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
