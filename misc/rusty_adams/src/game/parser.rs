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

    Ok(super::Game { header: header })
}

/// Parses the header of the game file.
fn parse_header(stream: &mut tokenizer::Stream) -> Result<super::Header, ParseError> {
    Ok(super::Header {
        unknown0: read_header_field(stream)?,
        num_items: read_header_field(stream)? + 1, // adjust for option base 0
        num_actions: read_header_field(stream)? + 1, // adjust for option base 0
        num_words: read_header_field(stream)?,     // adjust for option base 0
        num_rooms: read_header_field(stream)?,     // adjust for option base 0
        max_inventory: read_header_field(stream)?,
        starting_room: read_header_field(stream)?,
        num_treasures: read_header_field(stream)?,
        word_length: read_header_field(stream)?,
        light_duration: read_header_field(stream)?,
        num_messages: read_header_field(stream)?, // adjust for option base 0
        treasure_room: read_header_field(stream)?,
    })
}

fn read_header_field(stream: &mut tokenizer::Stream) -> Result<i32, ParseError> {
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
