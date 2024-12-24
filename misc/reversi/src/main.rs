use console::Term;
use std::io;

// Screen encapsulates the display and input for the game, and all output must
// be done through a singleton instance of Screen rather than stdout for
// flushing to work properly.
//
// A Screen does *not* encapsulate an instance of Board, but rather borrows one
// as needed for I/O.
struct Screen {
    term: Term,
}

impl Screen {
    const BLACK: u8 = 30;
    const RED: u8 = 31;
    const GREEN: u8 = 32;
    const YELLOW: u8 = 33;
    const BLUE: u8 = 34;
    //const MAGENTA: u8 = 35;
    //const CYAN: u8 = 36;
    const WHITE: u8 = 37;
    
    const GRAY: u8 = Screen::BLACK + 60;
    const LT_RED: u8 = Screen::RED + 60;
    //const LT_GREEN: u8 = Screen::GREEN + 60;
    const LT_YELLOW: u8 = Screen::YELLOW + 60;
    const LT_BLUE: u8 = Screen::BLUE + 60;
    //const LT_MAGENTA: u8 = Screen::MAGENTA + 60;
    //const LT_CYAN: u8 = Screen::CYAN + 60;
    const LT_WHITE: u8 = Screen::WHITE + 60;

    // new creates a new Screen instance.
    fn new() -> Self {
        Self { term: Term::stdout() }
    }

    // clear_screen clears the screen, homes the cursor, and sets the current
    // color to bright white.
    fn clear_screen(&mut self) -> io::Result<()> {
        self.term.write_str(format!("\x1b[2J\x1b[H\x1b[{}m", Self::LT_WHITE).as_str())
    }

    // draw_board draws the given Board on the screen.
    fn draw_board(&mut self, board: &Board) -> io::Result<()> {
        self.clear_screen()?;
        self.draw_box(2, 1, 19, 10, Self::GRAY)?;
        self.draw_text(4, 0, Self::GREEN, "1 2 3 4 5 6 7 8")?;        
        self.draw_text(4, 11, Self::GREEN, "1 2 3 4 5 6 7 8")?;    
        for y in 0..8 {
            let ch = ((y as u8)+97) as char;
            self.draw_text(0, y+2, Self::GREEN, format!("{}", ch).as_str())?;
            self.draw_text(22, y+2, Self::GREEN, format!("{}", ch).as_str())?;
        }

        for row in 0..8 {
            for col in 0..8 {
                let x = col*2 + 4;
                let y = row + 2;
                match board.get(col, row) {
                    Board::EMPTY => {
                        let sym = if board.is_valid(col, row, Board::HUMAN) { "?" } else { "·" };
                        self.draw_text(x, y, Self::WHITE, sym)?;
                    },
                    Board::HUMAN => self.draw_text(x, y, Self::LT_RED, "⦁")?,
                    Board::COMPUTER => self.draw_text(x, y, Self::LT_BLUE, "⦁")?,
                    _ => panic!("Internal error in board state")
                }
            }
        }

        let human = format!("Human:    {}", board.get_score(Board::HUMAN).unwrap());
        self.draw_text(28, 2, Self::LT_RED, human.as_str())?;

        let computer = format!("Computer: {}", board.get_score(Board::COMPUTER).unwrap());
        self.draw_text(28, 3, Self::LT_BLUE, computer.as_str())
    }

    // draw_box draws a box in the given color and at the given 0-based (x, y)
    // coordinates.
    fn draw_box(&mut self, x: i32, y: i32, width: i32, height: i32, color: u8) -> io::Result<()> {
        if width < 2 || height < 2 {
            return Ok(());
        }

        self.set_color(color)?;
        self.goto_xy(x, y)?;
        self.term.write_str("┌")?;
        for _ in 1..=(width - 2) {
            self.term.write_str("─")?;
        }
        self.term.write_str("┐")?;

        for y_offset in 1..=(height - 2) {
            self.goto_xy(x, y + y_offset)?;
            self.term.write_str("│")?;
            self.goto_xy(x + width - 1, y + y_offset)?;
            self.term.write_str("│")?;
        }

        self.goto_xy(x, y + height - 1)?;
        self.term.write_str("└")?;
        for _ in 1..=(width - 2) {
            self.term.write_str("─")?;
        }
        self.term.write_str("┘")
    }

    // draw_text draws text in the given color and at the given 0-based (x, y)
    // coordinates.
    fn draw_text(&mut self, x: i32, y: i32, color: u8, text: &str) -> io::Result<()> {
        self.goto_xy(x, y)?;
        self.set_color(color)?;
        self.term.write_str(text)
    }

    // goto_xy moves the cursor to the given 0-based (x, y) coordinates.
    fn goto_xy(&mut self, x: i32, y: i32) -> io::Result<()> {
        self.term.write_str(format!("\x1b[{};{}H", y+1, x+1).as_str())
    }

    // read_move reads a row (a-h) and column (1-8) from the user and
    // translates it into a zero-based (col, row) tuple.  Only valid moves
    // are accepted.
    fn read_move(&mut self, board: &Board) -> Option<(i32, i32)> {
        loop {
            self.draw_board(board).unwrap_or(());
            self.draw_text(28, 6, Self::WHITE, "Row (a-h)? ").unwrap_or(());
            let mut row = -1;
            while row == -1 {
                let ch = self.term.read_char().expect("Terminal error");
                if ch == 'q' {
                    return None;
                } else if (ch >= 'a') && (ch <= 'h') {
                    row = (ch as i32) - ('a' as i32);
                    self.draw_text(39, 6, Self::LT_WHITE, format!("{}", ch).as_str()).unwrap_or(());
                }
            };

            let mut col = -1;
            self.draw_text(28, 7, Self::WHITE, "Col (1-8)? ").unwrap_or(());
            while col == -1 {
                let ch = self.term.read_char().expect("Terminal error");
                if ch == 'q' {
                    return None;
                } else if (ch >= '1') && (ch <= '8') {
                    col = (ch as i32) - ('1' as i32);
                    self.draw_text(39, 7, Self::LT_WHITE, format!("{}", ch).as_str()).unwrap_or(());
                }
            };

            if board.is_valid(col, row, Board::HUMAN) {
                return Some((col, row));
            }
            self.draw_text(28, 9, Self::LT_YELLOW, "Invalid move!").unwrap_or(());
            self.term.read_char().expect("Terminal error");
        }
    }

    // set_color sets the current terminal color to the one given.
    fn set_color(&mut self, color: u8) -> io::Result<()> {
        self.term.write_str(format!("\x1b[{}m", color).as_str())
    }
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

impl Board {
    const EMPTY: u8 = 0;
    const HUMAN: u8 = 1;
    const COMPUTER: u8 = 2;
    
    const OFFSETS: [i32; 8] = [ 1, -7, -8, -9, -1, 7, 8, 9 ];
    
    // new returns a new Board with the initial pieces placed.
    fn new() -> Self {
        let mut board = Self {
            squares: [Self::EMPTY; 64],
            scores: [2, 2]
        };
        board.set(3, 3, Self::HUMAN);
        board.set(4, 4, Self::HUMAN);
        board.set(3, 4, Self::COMPUTER);
        board.set(4, 3, Self::COMPUTER);
        board
    }

    // get returns the value of the square at the given column and row.
    fn get(&self, col: i32, row: i32) -> u8 {
        self.squares[(row*8 + col) as usize]
    }

    // get_score returns the score for the given player.
    fn get_score(&self, player: u8) -> Option<i32> {
        if player == Self::HUMAN || player == Self::COMPUTER {
            Some(self.scores[(player-1) as usize])
        } else {
            None
        }
    }

    // set sets the square at the given column and row to the given value.
    fn set(&mut self, col: i32, row: i32, value: u8) {
        self.squares[(row*8 + col) as usize] = value;
    }

    // is_valid checks whether the given (column, row) is a valid move for the
    // given player.
    fn is_valid(&self, col: i32, row: i32, player: u8) -> bool {
        if self.get(col, row) == Self::EMPTY {
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
            this += Self::OFFSETS[dir as usize];
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
}

fn main() {
    let mut board = Board::new();
    let mut screen = Screen::new();
    loop {
        match screen.read_move(&board) {
            Some((col, row)) => println!("MOVE TO {}, {}", col, row),
            None => break
        }
    }
}
