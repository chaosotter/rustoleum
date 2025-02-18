//! This module contains all of the code used to initialize a Game structure
//! from a collection of tokens.

use regex::Regex;
use std::fmt::{Display, Error, Formatter};

use super::*;
use crate::tokenizer::Stream;

/// Initializes a new Game structure from a stream of tokens.
pub fn parse_game(stream: &mut Stream) -> Result<Game, ParseError> {
    let header = parse_header(stream)?;
    let mut actions = parse_actions(stream, header.num_actions)?;
    let words = parse_words(stream, header.num_words)?;
    let rooms = parse_rooms(stream, header.num_rooms)?;
    let messages = parse_messages(stream, header.num_messages)?;
    let items: Vec<Item> = parse_items(stream, header.num_items)?;
    parse_comments(stream, &mut actions)?;
    let footer = parse_footer(stream)?;

    Ok(Game {
        header,
        actions,
        verbs: words.0,
        nouns: words.1,
        rooms,
        messages,
        items,
        footer,
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
        actions.push(parse_action(stream)?);
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

    let conditions = [
        parse_condition(stream)?,
        parse_condition(stream)?,
        parse_condition(stream)?,
        parse_condition(stream)?,
        parse_condition(stream)?,
    ];
    
    let mut actions = [(); 4].map(|_| ActionType::default());
    for i in 0..2 {
        let num = _read_int(stream)?;
        actions[i * 2] = num / 150;
        actions[i * 2 + 1] = num % 150;
    }

    Ok(Action {
        verb_index,
        noun_index,
        conditions,
        actions,
        comment: None, // comments are after items in the game file
    })
}

/// Parses a single condition from the game file.  Each condition is expressed
/// condition type + (20 * parameter).
fn parse_condition(stream: &mut Stream) -> Result<Condition, ParseError> {
    let num = _read_int(stream)?;
    let param = num / 20;
    match num % 20 {
        0 => Ok(Condition::Parameter(param)),
        1 => Ok(Condition::ItemCarried(param)),
        2 => Ok(Condition::ItemInRoom(param)),
        3 => Ok(Condition::ItemPresent(param)),
        4 => Ok(Condition::PlayerInRoom(param)),
        5 => Ok(Condition::ItemNotInRoom(param)),
        6 => Ok(Condition::ItemNotCarried(param)),
        7 => Ok(Condition::PlayerNotInRoom(param)),
        8 => Ok(Condition::BitSet(param)),
        9 => Ok(Condition::BitClear(param)),
        10 => Ok(Condition::InventoryNotEmpty(param)),
        11 => Ok(Condition::InventoryEmpty(param)),
        12 => Ok(Condition::ItemNotPresent(param)),
        13 => Ok(Condition::ItemInGame(param)),
        14 => Ok(Condition::ItemNotInGame(param)),
        15 => Ok(Condition::CounterLE(param)),
        16 => Ok(Condition::CounterGE(param)),
        17 => Ok(Condition::ItemMoved(param)),
        18 => Ok(Condition::ItemNotMoved(param)),
        19 => Ok(Condition::CounterEQ(param)),
        _ => return Err(ParseError { msg: format!("Invalid condition (type {}, parameter {}", num % 20, param) })
    }
}

/// Parses all of the words from the game file, which are an interleaved array
/// of strings.  An initial "*" indicates a synonym.
fn parse_words(stream: &mut Stream, num_words: i32) -> Result<(Vec<Word>, Vec<Word>), ParseError> {
    let mut verbs = Vec::new();
    let mut nouns = Vec::new();
    for _ in 0..num_words {
        let verb = _read_word(stream)?;
        verbs.push(Word {
            word: verb.0,
            is_synonym: verb.1,
        });
        let noun = _read_word(stream)?;
        nouns.push(Word {
            word: noun.0,
            is_synonym: noun.1,
        });
    }
    Ok((verbs, nouns))
}

/// Parses all of the rooms from the game file.
fn parse_rooms(stream: &mut Stream, num_rooms: i32) -> Result<Vec<Room>, ParseError> {
    let mut rooms = Vec::new();
    for _ in 0..num_rooms {
        rooms.push(parse_room(stream)?);
    }
    Ok(rooms)
}

/// Parses a single room, which consists of six directions (north, south, east,
/// west, up, down) followed by a description. The description starts with "*"
/// to indicate that it stands alone, with no "I'm in a" prefix.
fn parse_room(stream: &mut Stream) -> Result<Room, ParseError> {
    let mut exits = [(); 6].map(|_| 0);
    for exit in &mut exits {
        *exit = _read_int(stream)?;
    }

    let desc = _read_word(stream)?;
    Ok(Room {
        exits,
        description: desc.0,
        is_literal: desc.1,
    })
}

// Parses all of the messages from the game file.
fn parse_messages(stream: &mut Stream, num_messages: i32) -> Result<Vec<String>, ParseError> {
    let mut messages = Vec::new();
    for _ in 0..num_messages {
        messages.push(_read_str(stream)?);
    }
    Ok(messages)
}

// Parses all of the items from the game file.
fn parse_items(stream: &mut Stream, num_items: i32) -> Result<Vec<Item>, ParseError> {
    let mut items = Vec::new();
    for _ in 0..num_items {
        items.push(parse_item(stream)?);
    }
    Ok(items)
}

/// Parses a single item, which consists of a string description and a room
/// number indicating the initial location.
///
/// Treasures are indicated with a leading "*", but unlike words and room
/// descriptions, we do not strip that prefix from the description.
///
/// If the description has a suffix of `/XXX/``, then automatic GET and DROP
/// operations can be performed using "XXX" as a noun.
fn parse_item(stream: &mut Stream) -> Result<Item, ParseError> {
    let mut description = _read_str(stream)?;
    let location = _read_int(stream)?;

    let is_treasure = description.starts_with("*");

    let re = Regex::new(r"^(?<description>.*)/(?<autograb>.*)/$").unwrap();
    let autograb = if let Some(caps) = re.captures(&description.clone()) {
        description = caps["description"].to_string();
        Some(caps["autograb"].to_string())
    } else {
        None
    };
    Ok(Item {
        description,
        location,
        is_treasure,
        autograb,
    })
}

/// Parses all of the comments from the game file, which are stored in the
/// actions.
fn parse_comments(stream: &mut Stream, actions: &mut Vec<Action>) -> Result<(), ParseError> {
    for action in actions {
        let comment = _read_str(stream)?;
        if !comment.is_empty() {
            action.comment = Some(comment);
        }
    }
    Ok(())
}

/// Parses the footer.
fn parse_footer(stream: &mut Stream) -> Result<Footer, ParseError> {
    Ok(Footer {
        version: _read_int(stream)?,
        adventure: _read_int(stream)?,
        magic: _read_int(stream)?,
    })
}

/// Reads in the next integer token.
fn _read_int(stream: &mut Stream) -> Result<i32, ParseError> {
    match stream.next_int() {
        Ok(value) => Ok(value),
        Err(e) => Err(ParseError { msg: format!("{}", e) }),
    }
}

/// Reads in the next string token.
fn _read_str(stream: &mut Stream) -> Result<String, ParseError> {
    match stream.next_str() {
        Ok(value) => Ok(value),
        Err(e) => Err(ParseError { msg: format!("{}", e) }),
    }
}

/// Reads in the next word.  A word is distinguished from a string token by
/// having an optional "*" prefix to indicate special handling.
fn _read_word(stream: &mut Stream) -> Result<(String, bool), ParseError> {
    let mut word = _read_str(stream)?;
    let has_prefix = word.starts_with("*");
    if has_prefix {
        word = word.strip_prefix("*").unwrap().to_string();
    }
    Ok((word, has_prefix))
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
