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

const EMPTY: u8 = 0;
const HUMAN: u8 = 1;
const COMPUTER: u8 = 2;

const OFFSETS: [i32; 8] = [ 1, -7, -8, -9, -1, 7, 8, 9 ];

// Board represents the state of the board.  For ease of coding, we use
// a single-dimensional array of 64 elements, each of which is 0 for empty,
// 1 for the human player, or 2 for the computer player.
//
// We keep track of the score explicitly simply for the sake of efficiency.
struct Board {
    squares: [u8; 64],
    scores: [i32; 2]
}

impl Board {
    // new returns a new Board with the initial pieces placed.
    fn new() -> Self {
        let mut board = Board {
            squares: [EMPTY; 64],
            scores: [2, 2]
        };
        board.set(3, 3, HUMAN);
        board.set(4, 4, HUMAN);
        board.set(3, 4, COMPUTER);
        board.set(4, 3, COMPUTER);
        board
    }

    // draw draws the board on the screen.
    fn draw(&self) {
        clear_screen();
        draw_box(2, 1, 19, 10, GRAY);
        draw_text(4, 0, GREEN, "1 2 3 4 5 6 7 8");        
        draw_text(4, 11, GREEN, "1 2 3 4 5 6 7 8");    
        for y in 0..8 {
            let ch = ((y as u8)+97) as char;
            draw_text(0, y+2, GREEN, &format!("{}", ch));
            draw_text(22, y+2, GREEN, &format!("{}", ch));
        }

        for row in 0..8 {
            for col in 0..8 {
                let x = col*2 + 4;
                let y = row + 2;
                match self.get(col, row) {
                    EMPTY => draw_text(x, y, WHITE, if self.is_valid(col, row, HUMAN) { "?" } else { "·" }),
                    HUMAN => draw_text(x, y, LT_RED,"⦁"),
                    COMPUTER => draw_text(x, y, LT_BLUE, "⦁"),
                    _ => panic!("Internal error in board state")
                }
            }
        }

        draw_text(28, 2, LT_RED, format!("Human:    {}", self.scores[0]).as_str());
        draw_text(28, 3, LT_BLUE, format!("Computer: {}", self.scores[1]).as_str());
    }

    // get returns the value of the square at the given column and row.
    fn get(&self, col: i32, row: i32) -> u8 {
        self.squares[(row*8 + col) as usize]
    }

    // set sets the square at the given column and row to the given value.
    fn set(&mut self, col: i32, row: i32, value: u8) {
        self.squares[(row*8 + col) as usize] = value;
    }

    // is_valid checks whether the given (column, row) is a valid move for the
    // given player.
    fn is_valid(&self, col: i32, row: i32, player: u8) -> bool {
        if self.get(col, row) == EMPTY {
            for dir in 0..8 {
                if self.is_valid_dir(col, row, player, dir) {
                    return true;
                }
            }
        }
        false
    }

    // is_valid_dir checks whether the given (column, row) is a valid move for
    // the given player in the given direction.  We assume that we have already
    // checked that the space is empty.
    fn is_valid_dir(&self, col: i32, row: i32, player: u8, dir: i32) -> bool {
        let other = player ^ 0b11;  // 1 -> 2, 2 -> 1
        let mut this = row*8 + col;
        let mut found = false;
        loop {
            this += OFFSETS[dir as usize];
            if (this < 0) || (this >= 64) {
                return false;
            }
            let val = self.squares[this as usize];
            if val == player {
                return found;
            } else if val == other {
                found = true;
            } else {
                return false;
            }
        }
    }

    fn get_move(&mut self, term: &mut Term) -> Option<i32> {
        loop {
            self.draw();
            draw_text(28, 6, WHITE, "Row (a-h)? ");
            term.flush().expect("Terminal error");
            let mut row = -1;
            while row == -1 {
                let ch = term.read_char().expect("Terminal error");
                if ch == 'q' {
                    return None;
                } else if (ch >= 'a') && (ch <= 'h') {
                    row = (ch as i32) - ('a' as i32);
                    draw_text(39, 6, LT_WHITE, format!("{}", ch).as_str());
                }
            };

            let mut col = -1;
            draw_text(28, 7, WHITE, "Col (1-8)? ");
            term.flush().expect("Terminal error");
            while col == -1 {
                let ch = term.read_char().expect("Terminal error");
                if ch == 'q' {
                    return None;
                } else if (ch >= '1') && (ch <= '8') {
                    col = (ch as i32) - ('1' as i32);
                    draw_text(39, 7, LT_WHITE, format!("{}", ch).as_str());
                }
            };

            if self.is_valid(col, row, HUMAN) {
                return Some(row*8 + col);
            }
            draw_text(28, 9, LT_YELLOW, "Invalid move!");
            term.read_char().expect("Terminal error");
        }
    }
}

fn main() {
    let mut board = Board::new();
    let mut term = Term::stdout();
    loop {
        match board.get_move(&mut term) {
            Some(loc) => println!("MOVE TO {}", loc),
            None => break
        }
    }
}
