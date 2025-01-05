//! This module contains all of the code used to write a GAme structure in the
//! same format as it was orginally read.
//! 
//! We use this primarily to test the correctness of our parsing, but it would
//! be reasonable to evolve this toward support for interactive game
//! modification.

use std::io::Write;

use crate::game::Game;

/// Writes a Game to the given writer.
///
/// This is the inverse of the `parse_game` function.
pub fn write_game<W: Write>(mut writer: W, game: &super::Game) -> std::io::Result<()> {
    write_header(writer, &game.header)?;
    Ok(())
}

/// Writes the header of a Game to the given writer.
fn write_header<W: Write>(mut writer: W, header: &super::Header) -> std::io::Result<()> {
    writeln!(writer, " {} ", header.unknown0)?;
    writeln!(writer, " {} ", header.num_items - 1)?;
    writeln!(writer, " {} ", header.num_actions - 1)?;
    writeln!(writer, " {} ", header.num_words - 1)?;
    writeln!(writer, " {} ", header.num_rooms - 1)?;
    writeln!(writer, " {} ", header.max_inventory)?;
    writeln!(writer, " {} ", header.starting_room)?;
    writeln!(writer, " {} ", header.num_treasures)?;
    writeln!(writer, " {} ", header.word_length)?;
    writeln!(writer, " {} ", header.light_duration)?;
    writeln!(writer, " {} ", header.num_messages - 1)?;
    writeln!(writer, " {} ", header.treasure_room)?;
    Ok(())    
}
