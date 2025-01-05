//! This module is the heart of the interpreter and contains the core data
//! structures for a Scott Adams game and the methods that manipulate it.
//! 
//! We keep actual I/O strictly partitioned away from this module, as the
//! intention is to make this whole mess work with WebAssembly at some point
//! after I learn it.

mod parser;

use crate::tokenizer;

/// Used in the `light_duration` field of the game header to indicate that the
/// light source never expires.
const ETERNAL_LIGHT: i32 = -1;

/// Defines the game itself.
#[derive(Debug)]
pub struct Game {
    /// The header of the game file.
    header: Header,
}

impl Game {
    /// Parses a new game from the given stream of tokens.
    pub fn new(stream: &mut tokenizer::Stream) -> Result<Game, parser::ParseError> {
        parser::parse_game(stream)
    }
}

/// Defines the header of a game file.
#[derive(Debug)]
struct Header {
    /// Unknown purpose.
    unknown0: i32,
    /// Number of items.
    num_items: i32,
    /// Number of actions.
    num_actions: i32,
    /// Number of both nouns and verbs.
    num_words: i32,
    /// Number of rooms.
    num_rooms: i32,
    /// Maximum number of inventory items.
    max_inventory: i32,
    /// 0-based index of initial room.
    starting_room: i32,
    /// Number of treasures (technically redundant).
    num_treasures: i32,
    /// Word length (3, 4, 5).
    word_length: i32,
    /// Number of turns for light, or -1 for eternal.
    light_duration: i32,
    /// Number of messages.
    num_messages: i32,
    /// 0-based index of treasure room for scoring.
    treasure_room: i32,
}
