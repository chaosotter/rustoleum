use console::Term;
use std::io::Write;

const COLORS: [&str; 7] = [
    "\x1b[31m",
    "\x1b[32m",
    "\x1b[33m",
    "\x1b[34m",
    "\x1b[35m",
    "\x1b[36m",
    "\x1b[37m"
];

type Board = [Vec<i32>; 3];

fn print_board(board: &Board) {
    print!("\x1b[2J\x1b[H\x1b[97m");
    for _ in 0..3 {
        print!("         ╭╮       ");
    }
    println!();

    for row in 0..7 {
        for col in 0..3 {
            if row < 7 - board[col].len() {
                print!("\x1b[97m         ││       ");
                continue;
            }
            let elem = board[col].len() - (7 - row);
            let disc = board[col][elem];
            for _ in 0..(7-disc+2) { print!(" "); }
            print!("{}", COLORS[disc as usize]);
            for _ in 0..(disc*2+2) { print!("▓"); }
            for _ in 0..(7-disc) { print!(" "); }
        }
        println!();
    }
    print!("\x1b[97m");
    for _ in 0..56 { print!("▔"); }
    println!();
}

fn main() {
    println!("Towers of Hanoi");
    println!();

    let discs = loop {
        println!("Enter number of discs (1-7): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Could not read line");
        match input.trim().parse::<i32>() {
            Ok(n) => if n >= 1 && n <= 7 { break n }
            Err(_) => { continue }
        }
    };

    let mut board: Board = [vec![], vec![], vec![]];
    for i in 0..discs {
        board[0].push(i);
    }

    let mut term = Term::stdout();
    while board[2].len() < (discs as usize) {
        print_board(&board);

        write!(term, "\x1b[37mMove from ").expect("Terminal error");
        let from: i32 = loop {
            let ch = term.read_char().expect("Terminal error");
            if ch >= '1' && ch <= '3' {
                break (ch as i32) - ('1' as i32)
            } else if ch == 'q' {
                break -1
            }
        };
        if from < 0 {
            break;
        }

        write!(term, "\x1b[97m{}\x1b[37m to ", from+1).expect("Terminal error");
        let to: i32 = loop {
            let ch = term.read_char().expect("Terminal error");
            if ch >= '1' && ch <= '3' {
                break (ch as i32) - ('1' as i32)
            } else if ch == 'q' {
                break -1
            }
        };
        if to < 0 {
            break;
        }
    }
}
