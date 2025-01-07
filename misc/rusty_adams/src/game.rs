//! This module is the heart of the interpreter and contains the core data
//! structures for a Scott Adams game and the methods that manipulate it.
//!
//! We keep actual I/O strictly partitioned away from this module, as the
//! intention is to make this whole mess work with WebAssembly at some point
//! after I learn it.

mod parser;
pub mod writer;

use crate::tokenizer;

/// Used in the `light_duration` field of the game header to indicate that the
/// light source never expires.
const ETERNAL_LIGHT: i32 = -1;

/// Defines the game itself.
#[derive(Debug)]
pub struct Game {
    pub header: Header,
    pub actions: Vec<Action>,
    pub verbs: Vec<Word>,
    pub nouns: Vec<Word>,
    pub rooms: Vec<Room>,
}

impl Game {
    /// Parses a new game from the given stream of tokens.
    pub fn new(stream: &mut tokenizer::Stream) -> Result<Game, parser::ParseError> {
        parser::parse_game(stream)
    }

    /// Prints a version of the game to stdout for debugging.
    pub fn print_debug(&self) {
        println!("{:?}", self.header);
        for (i, action) in self.actions.iter().enumerate() {
            println!("Action {}: {:?}", i, action);
        }
        for (i, verb) in self.verbs.iter().enumerate() {
            println!("Verb {}: {:?}", i, verb);
        }
        for (i, noun) in self.nouns.iter().enumerate() {
            println!("Noun {}: {:?}", i, noun);
        }
        for (i, room) in self.rooms.iter().enumerate() {
            println!("Room {}: {:?}", i, room);
        }
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

/// Defines a single action.
#[derive(Debug)]
struct Action {
    /// The verb index.
    verb_index: i32,
    /// The noun index.
    noun_index: i32,
    /// The conditions (five in all).
    conditions: [Condition; 5],
    /// The actions (four in all).
    actions: [ActionType; 4],
}

/// Defines a single condition.
#[derive(Debug, Default)]
struct Condition {
    /// The condition type.
    cond_type: ConditionType,
    /// The condition value.
    value: i32,
}

/// Defines a condition type.
/// TODO: Redesign Condition and ConditionType to be a single enum.
type ConditionType = i32;

/// Defines an action type.
#[derive(Debug, Default)]
enum ActionType {
    #[default]
    Unknown,
    Generic(i32),
}

/// Defines a word (either a verb or a noun).
#[derive(Debug, Default)]
struct Word {
    /// The word text (truncated to the word length)
    word: String,
    /// Indicates this word is a synonym of the previous word.
    is_synonym: bool,
}

/// Defines a room.
#[derive(Debug)]
struct Room {
    /// The room description.
    description: String,
    /// Indicates the description is to be printed literally (no "I'm in a" prefix).
    is_literal: bool,
    /// The room exits.
    exits: [i32; 6],
}
