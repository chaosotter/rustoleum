use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn print_file(path: &String) -> Result<(), String> {
    let file = File::open(&path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            return Err(format!("Error opening file: {}", error));
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                return Err(format!("Error reading file: {}", error));
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {} <filename>", args[0]);
    }
    let path = &args[1];

    if let Err(error) = print_file(path) {
        panic!("{}", error);
    }
}
