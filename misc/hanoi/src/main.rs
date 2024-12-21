use console::Term;
use std::io::Write;

// Returns an ANSI terminal command to set the foreground color (0-7).
fn color(color: i32) -> String {
    return format!("\x1b[3{}m", color);
}

// The board consists of three stacks of discs.
struct Board {
    discs: i32,
    stacks: [Vec<i32>; 3]
}

impl Board {
    fn new(discs: i32) -> Self { 
        let mut board = Self{discs: discs, stacks: [vec![], vec![], vec![]]};
        for i in 0..discs {
            board.stacks[0].push(i);
        }
        board
    }

    fn draw(&self) {
        print!("\x1b[2J\x1b[H\x1b[97m");
        for _ in 0..3 {
            print!("         ╭╮       ");
        }
        println!();

        for row in 0..7 {
            for col in 0..3 {
                if row < 7 - self.stacks[col].len() {
                    print!("\x1b[97m         ││       ");
                    continue;
                }
                let elem = self.stacks[col].len() - (7 - row);
                let disc = self.stacks[col][elem];
                for _ in 0..(7-disc+2) { print!(" "); }
                print!("{}", color(disc + 1));
                for _ in 0..(disc*2+2) { print!("▓"); }
                for _ in 0..(7-disc) { print!(" "); }
            }
            println!();
        }
        print!("\x1b[97m");
        for _ in 0..56 { print!("▔"); }
        println!();
    }

    fn move_disc(&mut self, from: usize, to: usize) -> bool {
        if from == to {
            return false;
        } else if self.stacks[from].len() == 0 {
            return false;
        } else if self.stacks[to].len() > 0 {
            let from_size = self.stacks[from][0];
            let to_size = self.stacks[to][0];
            if from_size > to_size {
                return false
            }
        }

        let disc = self.stacks[from].remove(0);
        self.stacks[to].insert(0, disc);
        true
    }

    fn won(&self) -> bool {
        self.stacks[2].len() == (self.discs as usize)
    }
}

fn get_post(term: &Term) -> Option<i32> {
    loop {
        let ch = term.read_char().expect("Terminal error");
        if ch >= '1' && ch <= '3' {
            return Some((ch as i32) - ('1' as i32));
        } else if ch == 'q' {
            return None
        }
    }
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

    let mut board = Board::new(discs);
    let mut term = Term::stdout();

    while !board.won() {
        board.draw();

        write!(term, "\x1b[37mMove from ").expect("Terminal error");
        let from = match get_post(&term) {
            Some(disc) => disc,
            None => break
        };
        
        write!(term, "\x1b[97m{}\x1b[37m to ", from+1).expect("Terminal error");
        let to: i32 = match get_post(&term) {
            Some(disc) => disc,
            None => break
        };

        board.move_disc(from as usize, to as usize);
    }

    if board.won() {
        println!("\x1b[2J\x1b[H\x1b[97mYou won!");
    }
}
