//! This module is the heart of the interpreter and contains the core data
//! structures for a Scott Adams game and the methods that manipulate it.
//!
//! We keep actual I/O strictly partitioned away from this module, as the
//! intention is to make this whole mess work with WebAssembly at some point
//! after I learn it.

mod parser;
pub mod writer;

use crate::tokenizer;
use std::fmt::{Display, Error, Formatter};

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

/// Defines a condition, which is a parameterized predicate.
#[derive(Debug)]
#[repr(i32)]
enum Condition {
    Parameter(i32) = 0,
    ItemCarried(i32) = 1,
    ItemInRoom(i32) = 2,
    ItemPresent(i32) = 3,
    PlayerInRoom(i32) = 4,
    ItemNotInRoom(i32) = 5,
    ItemNotCarried(i32) = 6,
    PlayerNotInRoom(i32) = 7,
    BitSet(i32) = 8,
    BitClear(i32) = 9,
    InventoryNotEmpty(i32) = 10,
    InventoryEmpty(i32) = 11,
    ItemNotPresent(i32) = 12,
    ItemInGame(i32) = 13,
    ItemNotInGame(i32) = 14,
    CounterLE(i32) = 15,
    CounterGE(i32) = 16,
    ItemMoved(i32) = 17,
    ItemNotMoved(i32) = 18,
    CounterEQ(i32) = 19,
}

impl Display for Condition {
    /// Makes a condition human-readable for debugging.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Condition::Parameter(n) => write!(f, "Action parameter {}", n),
            Condition::ItemCarried(n) => write!(f, "Item {} carried?", n),
            Condition::ItemInRoom(n) => write!(f, "Item {} in room?", n),
            Condition::ItemPresent(n) => write!(f, "Item {} present?", n),
            Condition::PlayerInRoom(n) => write!(f, "Player in room {}?", n),
            Condition::ItemNotInRoom(n) => write!(f, "Item {} not in room?", n),
            Condition::ItemNotCarried(n) => write!(f, "Item {} not carried?", n),
            Condition::PlayerNotInRoom(n) => write!(f, "Player not in room {}?", n),
            Condition::BitSet(n) => write!(f, "Bit flag {} set?", n),
            Condition::BitClear(n) => write!(f, "Bit flag {} clear?", n),
            Condition::InventoryNotEmpty(n) => write!(f, "Something is carried ({} ignored)?", n),
            Condition::InventoryEmpty(n) => write!(f, "Nothing is carried ({} ignored)?", n),
            Condition::ItemNotPresent(n) => write!(f, "Item {} not present?", n),
            Condition::ItemInGame(n) => write!(f, "Item {} in the game?", n),
            Condition::ItemNotInGame(n) => write!(f, "Item {} not in the game?", n),
            Condition::CounterLE(n) => write!(f, "Current counter <= {}?", n),
            Condition::CounterGE(n) => write!(f, "Current counter >= {}?", n),
            Condition::ItemMoved(n) => write!(f, "Item {} moved?", n),
            Condition::ItemNotMoved(n) => write!(f, "Item {} not moved?", n),
            Condition::CounterEQ(n) => write!(f, "Current counter == {}?", n),
        }
    }
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
