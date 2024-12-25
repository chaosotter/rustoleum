use std::env;

#[derive(Debug)]
struct Size {
    bytes: u64,
    kilobytes: u64,
    megabytes: u64,
    gigabytes: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <size>", args[0]);
        return;
    }

    let mut parts = args[1].split(" ").collect::<Vec<&str>>();
    if parts.len() < 2 {
        parts.push("b");
    }
    if parts.len() != 2 {
        println!("Invalid size format. Please use the format: <number> <unit>");
        return;
    }

    let size = parts[0].parse::<u64>().unwrap();
    let units = parts[1].to_lowercase();
    let size = match units.as_str() {
        "b" => size,
        "kb" => size * 1024,
        "mb" => size * 1024 * 1024,
        "gb" => size * 1024 * 1024 * 1024,
        _ => {
            println!("Invalid units. Please use b, kb, mb, or gb.");
            return;
        }
    };

    let size = Size{
        bytes: size,
        kilobytes: size / 1024,
        megabytes: size / 1024 / 1024,
        gigabytes: size / 1024 / 1024 / 1024,
    }; 
    println!("{:?}", size);
}