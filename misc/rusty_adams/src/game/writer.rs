//! This module contains all of the code used to write a Game structure in the
//! same format as it was orginally read.
//!
//! We use this primarily to test the correctness of our parsing, but it would
//! be reasonable to evolve this toward support for interactive game
//! modification.

use std::io::Write;

use super::*;

/// Writes a Game to the given Writer.
///
/// This is the inverse of the `parse_game` function.
pub fn write_game<W: Write>(writer: &mut W, game: &Game) -> std::io::Result<()> {
    write_header(writer, &game.header)?;
    write_actions(writer, &game.actions)?;
    write_words(writer, &game.verbs, &game.nouns)?;
    write_rooms(writer, &game.rooms)?;
    write_messages(writer, &game.messages)?;
    write_items(writer, &game.items)?;
    write_comments(writer, &game.actions)?;
    write_footer(writer, &game.footer)?;
    Ok(())
}

/// Writes the header.
fn write_header<W: Write>(writer: &mut W, header: &Header) -> std::io::Result<()> {
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

/// Writes all actions.
fn write_actions<W: Write>(writer: &mut W, actions: &[Action]) -> std::io::Result<()> {
    for action in actions.iter() {
        write_action(writer, action)?;
    }
    Ok(())
}

/// Writes a single action.
fn write_action<W: Write>(writer: &mut W, action: &Action) -> std::io::Result<()> {
    writeln!(writer, " {} ", action.verb_index * 150 + action.noun_index)?;
    for cond in action.conditions.iter() {
        writeln!(writer, " {} ", cond.cond_type + 20 * cond.value)?;
    }
    for i in 0..2 {
        if let super::ActionType::Generic(a1) = action.actions[i * 2] {
            if let super::ActionType::Generic(a2) = action.actions[i * 2 + 1] {
                writeln!(writer, " {} ", a1 * 150 + a2)?;
            }
        }
    }
    Ok(())
}

/// Writes all words.
fn write_words<W: Write>(writer: &mut W, verbs: &[Word], nouns: &[Word]) -> std::io::Result<()> {
    assert!(verbs.len() == nouns.len());
    for i in 0..verbs.len() {
        write_word(writer, verbs.get(i).unwrap())?;
        write_word(writer, nouns.get(i).unwrap())?;
    }
    Ok(())
}

/// Writes a single word.
fn write_word<W: Write>(writer: &mut W, word: &Word) -> std::io::Result<()> {
    if word.is_synonym {
        writeln!(writer, r#""*{}""#, word.word)
    } else {
        writeln!(writer, r#""{}""#, word.word)
    }
}

/// Writes all rooms.
fn write_rooms<W: Write>(writer: &mut W, rooms: &[Room]) -> std::io::Result<()> {
    for room in rooms.iter() {
        write_room(writer, room)?;
    }
    Ok(())
}

/// Writes a single room.
fn write_room<W: Write>(writer: &mut W, room: &Room) -> std::io::Result<()> {
    for exit in room.exits.iter() {
        writeln!(writer, " {} ", exit)?;
    }
    if room.is_literal {
        writeln!(writer, r#""*{}""#, room.description)
    } else {
        writeln!(writer, r#""{}""#, room.description)
    }
}

/// Writes all messages.
fn write_messages<W: Write>(writer: &mut W, messages: &[String]) -> std::io::Result<()> {
    for message in messages.iter() {
        writeln!(writer, r#""{}""#, message)?
    }
    Ok(())
}

/// Writes all items.
fn write_items<W: Write>(writer: &mut W, items: &[Item]) -> std::io::Result<()> {
    for item in items.iter() {
        write_item(writer, item)?;
    }
    Ok(())
}

/// Writes a single item.
fn write_item<W: Write>(writer: &mut W, item: &Item) -> std::io::Result<()> {
    let description = if let Some(autograb) = &item.autograb {
        format!("{}/{}/", item.description, autograb)
    } else {
        item.description.clone()
    };
    writeln!(writer, r#""{}" {} "#, description, item.location)
}

/// Writes all comments.
fn write_comments<W: Write>(writer: &mut W, actions: &[Action]) -> std::io::Result<()> {
    for action in actions.iter() {
        match &action.comment {
            Some(comment) => writeln!(writer, r#""{}""#, comment)?,
            None => writeln!(writer, r#""""#)?,
        }
    }
    Ok(())
}

/// Writes the footer.
fn write_footer<W: Write>(writer: &mut W, footer: &Footer) -> std::io::Result<()> {
    writeln!(writer, " {} ", footer.version)?;
    writeln!(writer, " {} ", footer.adventure)?;
    writeln!(writer, " {} ", footer.magic)?;
    Ok(())
}
