//! We put most of our functionality here so that we can write integration
//! without difficulty.

use std::fs;

pub mod game;
mod tokenizer;

/// Loads a game from the given path.
pub fn load_game(path: &str) -> Result<game::Game, String> {
    let data = match fs::read(path) {
        Ok(data) => data,
        Err(err) => return Err(format!("Error: {}", err)),
    };

    let mut stream = match tokenizer::Stream::new(data) {
        Ok(stream) => stream,
        Err(err) => return Err(err.to_string()),
    };

    match game::Game::new(&mut stream) {
        Ok(game) => Ok(game),
        Err(err) => Err(err.to_string()),
    }
}
