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

/// Defines a condition, which is a parameterized predicate.
#[derive(Debug)]
enum Condition {
    Parameter(i32),
    ItemCarried(i32),
    ItemInRoom(i32),
    ItemPresent(i32),
    PlayerInRoom(i32),
    ItemNotInRoom(i32),
    ItemNotCarried(i32),
    PlayerNotInRoom(i32),
    BitSet(i32),
    BitClear(i32),
    InventoryNotEmpty(i32),
    InventoryEmpty(i32),
    ItemNotPresent(i32),
    ItemInGame(i32),
    ItemNotInGame(i32),
    CounterLE(i32),
    CounterGE(i32),
    ItemMoved(i32),
    ItemNotMoved(i32),
    CounterEQ(i32),
    Invalid(i32, i32),
}

impl Condition {
    /// Converts an integer expressed as (param * 20) + type into a Condition.
    pub fn from_i32(num: i32) -> Condition {
        let param = num / 20;
        match num % 20 {
            0 => Condition::Parameter(param),
            1 => Condition::ItemCarried(param),
            2 => Condition::ItemInRoom(param),
            3 => Condition::ItemPresent(param),
            4 => Condition::PlayerInRoom(param),
            5 => Condition::ItemNotInRoom(param),
            6 => Condition::ItemNotCarried(param),
            7 => Condition::PlayerNotInRoom(param),
            8 => Condition::BitSet(param),
            9 => Condition::BitClear(param),
            10 => Condition::InventoryNotEmpty(param),
            11 => Condition::InventoryEmpty(param),
            12 => Condition::ItemNotPresent(param),
            13 => Condition::ItemInGame(param),
            14 => Condition::ItemNotInGame(param),
            15 => Condition::CounterLE(param),
            16 => Condition::CounterGE(param),
            17 => Condition::ItemMoved(param),
            18 => Condition::ItemNotMoved(param),
            19 => Condition::CounterEQ(param),
            _ => Condition::Invalid(num % 20, param),
        }
    }

    /// Converts a Condition back into an integer.
    pub fn to_i32(&self) -> i32 {
        match self {
            Condition::Parameter(n) => 0 + (n * 20),
            Condition::ItemCarried(n) => 1 + (n * 20),
            Condition::ItemInRoom(n) => 2 + (n * 20),
            Condition::ItemPresent(n) => 3 + (n * 20),
            Condition::PlayerInRoom(n) => 4 + (n * 20),
            Condition::ItemNotInRoom(n) => 5 + (n * 20),
            Condition::ItemNotCarried(n) => 6 + (n * 20),
            Condition::PlayerNotInRoom(n) => 7 + (n * 20),
            Condition::BitSet(n) => 8 + (n * 20),
            Condition::BitClear(n) => 9 + (n * 20),
            Condition::InventoryNotEmpty(n) => 10 + (n * 20),
            Condition::InventoryEmpty(n) => 11 + (n * 20),
            Condition::ItemNotPresent(n) => 12 + (n * 20),
            Condition::ItemInGame(n) => 13 + (n * 20),
            Condition::ItemNotInGame(n) => 14 + (n * 20),
            Condition::CounterLE(n) => 15 + (n * 20),
            Condition::CounterGE(n) => 16 + (n * 20),
            Condition::ItemMoved(n) => 17 + (n * 20),
            Condition::ItemNotMoved(n) => 18 + (n * 20),
            Condition::CounterEQ(n) => 19 + (n * 20),
            Condition::Invalid(typ, n) => typ + (n * 20),
        }
    }
}

/// Defines the type of an action -- or rather, a subaction, as there are up to
/// four subactions associated with an action.
#[derive(Debug)]
enum ActionType {
    Nothing,
    Message(i32),
    GetItem,
    DropItem,
    MovePlayer,
    RemoveItem(bool), // true for RemoveItem2 (duplicate action type)
    SetDarkness,
    ClearDarkness,
    SetBit,
    ClearBit,
    Death,
    PutItem,
    GameOver,
    DescribeRoom(bool), // true for DescribeRoom2 (duplicate action type)
    Score,
    Inventory,
    SetBit0,
    ClearBit0,
    RefillLight,
    ClearScreen,
    SaveGame,
    SwapItems,
    Continue,
    TakeItem,
    MoveItemToItem,
    DecrementCounter,
    PrintCounter,
    SetCounter,
    SwapLocation,
    SelectCounter,
    AddToCounter,
    SubFromCounter,
    EchoNoun,
    EchoNounCR,
    EchoCR,
    SwapLocationN,
    Delay,
    DrawPicture,
    Invalid(i32),
}

impl ActionType {
    /// Converts an integer representing an action type into an ActionType.
    pub fn from_i32(num: i32) -> ActionType {
        match num {
            0 => ActionType::Nothing,
            1..=51 => ActionType::Message(num - 1),
            52 => ActionType::GetItem,
            53 => ActionType::DropItem,
            54 => ActionType::MovePlayer,
            55 => ActionType::RemoveItem(false),
            56 => ActionType::SetDarkness,
            57 => ActionType::ClearDarkness,
            58 => ActionType::SetBit,
            59 => ActionType::RemoveItem(true),
            60 => ActionType::ClearBit,
            61 => ActionType::Death,
            62 => ActionType::PutItem,
            63 => ActionType::GameOver,
            64 => ActionType::DescribeRoom(false),
            65 => ActionType::Score,
            66 => ActionType::Inventory,
            67 => ActionType::SetBit0,
            68 => ActionType::ClearBit0,
            69 => ActionType::RefillLight,
            70 => ActionType::ClearScreen,
            71 => ActionType::SaveGame,
            72 => ActionType::SwapItems,
            73 => ActionType::Continue,
            74 => ActionType::TakeItem,
            75 => ActionType::MoveItemToItem,
            76 => ActionType::DescribeRoom(true),
            77 => ActionType::DecrementCounter,
            78 => ActionType::PrintCounter,
            79 => ActionType::SetCounter,
            80 => ActionType::SwapLocation,
            81 => ActionType::SelectCounter,
            82 => ActionType::AddToCounter,
            83 => ActionType::SubFromCounter,
            84 => ActionType::EchoNoun,
            85 => ActionType::EchoNounCR,
            86 => ActionType::EchoCR,
            87 => ActionType::SwapLocationN,
            88 => ActionType::Delay,
            89 => ActionType::DrawPicture,
            102..=150 => ActionType::Message(num - 51),
            _ => ActionType::Invalid(num),
        }
    }
            
    /// Converts an ActionType back to an integer.
    pub fn to_i32(&self) -> i32 {
        match self {
            ActionType::Nothing => 0,
            ActionType::Message(num) => {
                match num {
                    0..=50 => num + 1,
                    51..=99 => num + 51,
                    _ => 0, // doesn't happen
                }
            },
            ActionType::GetItem => 52,
            ActionType::DropItem => 53,
            ActionType::MovePlayer => 54,
            ActionType::RemoveItem(dup) => if !dup { 55 } else { 59 },
            ActionType::SetDarkness => 56,
            ActionType::ClearDarkness => 57,
            ActionType::SetBit => 58,
            ActionType::ClearBit => 60,
            ActionType::Death => 61,
            ActionType::PutItem => 62,
            ActionType::GameOver => 63,
            ActionType::DescribeRoom(dup) => if !dup { 64 } else { 76 },
            ActionType::Score => 65,
            ActionType::Inventory => 66,
            ActionType::SetBit0 => 67,
            ActionType::ClearBit0 => 68,
            ActionType::RefillLight => 69,
            ActionType::ClearScreen => 70,
            ActionType::SaveGame => 71,
            ActionType::SwapItems => 72,
            ActionType::Continue => 73,
            ActionType::TakeItem => 74,
            ActionType::MoveItemToItem => 75,
            ActionType::DecrementCounter => 77,
            ActionType::PrintCounter => 78,
            ActionType::SetCounter => 79,
            ActionType::SwapLocation => 80,
            ActionType::SelectCounter => 81,
            ActionType::AddToCounter => 82,
            ActionType::SubFromCounter => 83,
            ActionType::EchoNoun => 84,
            ActionType::EchoNounCR => 85,
            ActionType::EchoCR => 86,
            ActionType::SwapLocationN => 87,
            ActionType::Delay => 88,
            ActionType::DrawPicture => 89,
            ActionType::Invalid(num) => *num,
        }
    }
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
