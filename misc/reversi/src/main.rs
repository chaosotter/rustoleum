use console::Term;

const BLACK: u8 = 30;
const RED: u8 = 31;
const GREEN: u8 = 32;
const YELLOW: u8 = 33;
const BLUE: u8 = 34;
const MAGENTA: u8 = 35;
const CYAN: u8 = 36;
const WHITE: u8 = 37;

const GRAY: u8 = BLACK + 60;
const LT_RED: u8 = RED + 60;
const LT_GREEN: u8 = GREEN + 60;
const LT_YELLOW: u8 = YELLOW + 60;
const LT_BLUE: u8 = BLUE + 60;
const LT_MAGENTA: u8 = MAGENTA + 60;
const LT_CYAN: u8 = CYAN + 60;
const LT_WHITE: u8 = WHITE + 60;

// clear_screen clears the screen, homes the cursor, and sets the current color
// to bright white.
fn clear_screen() {
    print!("\x1b[2J\x1b[H\x1b[{}m", LT_WHITE);
}

// draw_box draws a box in the given color and at the given (x, y) coordinates.
// The coordinates are zero-based.
fn draw_box(x: i32, y: i32, width: i32, height: i32, color: u8) {
    if width < 2 || height < 2 {
        return;
    }

    set_color(color);
    goto_xy(x, y);
    print!("┌");
    for _ in 1..=(width - 2) {
        print!("─");
    }
    print!("┐");

    for y_offset in 1..=(height - 2) {
        goto_xy(x, y + y_offset);
        print!("│");
        goto_xy(x + width - 1, y + y_offset);
        print!("│");
    }

    goto_xy(x, y + height - 1);
    print!("└");
    for _ in 1..=(width - 2) {
        print!("─");
    }
    print!("┘");
}

// draw_text draws text in the given color and at the given (x, y) coordinates.
// The coordinates are zero-based.
fn draw_text(x: i32, y: i32, color: u8, text: &str) {
    goto_xy(x, y);
    set_color(color);
    print!("{}", text);
}

// goto_xy moves the cursor to the given (x, y) coordinates.
// The coordinates are zero-based.
fn goto_xy(x: i32, y: i32) {
    print!("\x1b[{};{}H", y+1, x+1);
}

// set_color sets the current terminal color to the one given.
fn set_color(color: u8) {
    print!("\x1b[{}m", color);
}

// Board represents the state of the board.  For ease of coding, we use
// a single-dimensional array of 64 elements, each of which is 0 for empty,
// 1 for the human player, or 2 for the computer player.
//
// We keep track of the score explicitly simply for the sake of efficiency.
struct Board {
    squares: [u8; 64],
    scores: [i32; 2]
}

const Human: u8 = 1;
const Computer: u8 = 2;

impl Board {
    // new returns a new Board with the initial pieces placed.
    fn new() -> Self {
        let mut board = Board {
            squares: [0; 64],
            scores: [0, 0]
        };
        board.set(3, 3, Human);
        board.set(4, 4, Human);
        board.set(3, 4, Computer);
        board.set(4, 3, Computer);
        board
    }

    // draw draws the board on the screen.
    fn draw(&self) {
        clear_screen();
        draw_box(1, 1, 19, 10, GRAY);
        draw_text(3, 0, GREEN, "1 2 3 4 5 6 7 8");        
        draw_text(3, 11, GREEN, "1 2 3 4 5 6 7 8");        
    }

    // set sets the square at the given (x, y) coordinates to the given value.
    // The coordinates are zero-based.
    fn set(&mut self, x: i32, y: i32, value: u8) {
        self.squares[(y*8 + x) as usize] = value;
    }
}

fn main() {
    println!("Hello, world!");

    let mut board = Board::new();
    board.draw();

    let mut term = Term::stdout();
    loop {
        let ch = term.read_char().expect("Terminal error");
        if ch == 'q' {
            break;
        }
        println!("Char: [{}], Code: {}", ch, ch as i32);
    }
}
