use std::fs;

mod game;
mod tokenizer;

fn main() {
    let data = match fs::read("games/adv01.dat") {
        Ok(data) => data,
        Err(err) => panic!("Error: {}", err),
    };

    let mut stream = match tokenizer::Stream::new(data) {
        Ok(stream) => stream,
        Err(err) => panic!("{}", err.to_string()),
    };

    match game::Game::new(&mut stream) {
        Ok(game) => println!("{:?}", game),
        Err(err) => panic!("{}", err.to_string()),
    };
}
