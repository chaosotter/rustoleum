use std::fs;

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

    while !stream.done() {
        let token = stream.next_token();
        println!("{:?}", token);
    }
}
