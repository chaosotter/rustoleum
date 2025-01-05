//! This integration test makes sure that we are able to reverse the parsing
//! process and write a game back out in exactly the form it was read in.

use pretty_assertions::assert_eq;
use std::fs;

extern crate rusty_adams;

#[test]
fn test_parse_and_write_are_inverses() {
    let game = match rusty_adams::load_game("games/adv01.dat") {
        Ok(game) => game,
        Err(err) => panic!("Error: {}", err),
    };

    let mut got: Vec<u8> = Vec::new();
    match rusty_adams::game::writer::write_game(&mut got, &game) {
        Ok(_) => (),
        Err(err) => panic!("Error: {}", err),
    };

    let want = match fs::read("games/adv01.dat") {
        Ok(data) => data,
        Err(err) => panic!("Error: {}", err),
    };

    let got = String::from_utf8(got).unwrap();
    let want: String = String::from_utf8(want).unwrap();
    assert_eq!(got, want);
}
