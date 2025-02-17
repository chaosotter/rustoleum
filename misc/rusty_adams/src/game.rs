//! This module is the heart of the interpreter and contains the core data
//! structures for a Scott Adams game and the methods that manipulate it.
//!
//! We keep actual I/O strictly partitioned away from this module, as the
//! intention is to make this whole mess work with WebAssembly at some point
//! after I learn it.

mod parser;
pub mod writer;

use crate::tokenizer;

/// Used in the `light_duration` field of the `Header` struct` to indicate that
/// the light source never expires.
const ETERNAL_LIGHT: i32 = -1;

/// Used in the `location` field of the `Item` struct to indicate that the item
/// is in the player's inventory.
const INVENTORY: i32 = -1;

/// Defines the game itself.
#[derive(Debug)]
pub struct Game {
    header: Header,
    actions: Vec<Action>,
    verbs: Vec<Word>,
    nouns: Vec<Word>,
    rooms: Vec<Room>,
    messages: Vec<String>,
    items: Vec<Item>,
    footer: Footer,
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
        for (i, message) in self.messages.iter().enumerate() {
            println!("Message {}: {:?}", i, message);
        }
        for (i, item) in self.items.iter().enumerate() {
            println!("Item {}: {:?}", i, item);
        }
        println!("{:?}", self.footer);
    }
}

/// Defines the header.
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
    /// The comment (purely for documentation).
    comment: Option<String>,
}

/// Identifies a condition type.  We represent this as an i32 in preference to
/// representing conditions as an Enum because there's (AFAIK) no way to
/// initialize an Enum by discriminant, as we would logically do in the parser.
type ConditionType = i32;

const CONDITION_PARAMETER: ConditionType = 0;
const CONDITION_ITEM_CARRIED: ConditionType = 1;
const CONDITION_ITEM_IN_ROOM: ConditionType = 2;
const CONDITION_ITEM_PRESENT: ConditionType = 3;
const CONDITION_PLAYER_IN_ROOM: ConditionType = 4;
const CONDITION_ITEM_NOT_IN_ROOM: ConditionType = 5;
const CONDITION_ITEM_NOT_CARRIED: ConditionType = 6;
const CONDITION_PLAYER_NOT_IN_ROOM: ConditionType = 7;
const CONDITION_BIT_SET: ConditionType = 8;
const CONDITION_BIT_CLEAR: ConditionType = 9;
const CONDITION_INVENTORY_NOT_EMPTY: ConditionType = 10;
const CONDITION_INVENTORY_EMPTY: ConditionType = 11;
const CONDITION_ITEM_NOT_PRESENT: ConditionType = 12;
const CONDITION_ITEM_IN_GAME: ConditionType = 13;
const CONDITION_ITEM_NOT_IN_GAME: ConditionType = 14;
const CONDITION_COUNTER_LE: ConditionType = 15;
const CONDITION_COUNTER_GE: ConditionType = 16;
const CONDITION_ITEM_MOVED: ConditionType = 17;
const CONDITION_ITEM_NOT_MOVED: ConditionType = 18;
const CONDITION_COUNTER_EQ: ConditionType = 19;

/// Defines a single condition.
#[derive(Debug, Default)]
struct Condition {
    /// The condition type.
    cond_type: ConditionType,
    /// The condition value.
    value: i32,
}

/// Identifies an action type.  We represent this as an i32 in preference to
/// representing action types as an Enum because there's (AFAIK) no way to
/// initialize an Enum by discriminant, as we would logically do in the parser.
type ActionType = i32;

const ACTION_NOTHING: ActionType = 0;

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

/// Defines an item (object).
#[derive(Debug)]
struct Item {
    /// The item description.
    description: String,
    /// The item location (possibly `INVENTORY`).
    location: i32,
    /// Is the item a treasure (denoted by asterisks in the description)?
    is_treasure: bool,
    /// If set, automatic get/drop works, using this name.
    autograb: Option<String>,
}

/// Defines the footer.
#[derive(Debug)]
struct Footer {
    /// The version number.
    version: i32,
    /// The adventure number.
    adventure: i32,
    /// Magic number (purpose unknown).
    magic: i32,
}
