//! This module contains the Screen type, which encapsulates all of the I/O
//! for the game.  This is done with an unbuffered terminal so that we can
//! respond to individual keystrokes.

use console::Term;
use std::io;

use crate::board;

/// Screen encapsulates the display and input for the game.  All output must
/// be done through a singleton instance of Screen rather than stdout for
/// flushing to work properly.
///
/// A Screen does *not* encapsulate an instance of Board, but rather borrows one
/// as needed for I/O.
pub struct Screen {
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

    /// new creates a new Screen instance.
    pub fn new() -> Self {
        Self { term: Term::stdout() }
    }

    // clear_screen clears the screen, homes the cursor, and sets the current
    // color to bright white.
    fn clear_screen(&mut self) -> io::Result<()> {
        self.term.write_str(format!("\x1b[2J\x1b[H\x1b[{}m", Self::LT_WHITE).as_str())
    }

    // draw_board draws the given Board on the screen.
    pub fn draw_board(&mut self, board: &board::Board) -> io::Result<()> {
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
                let loc = row*8 + col;
                let x = col*2 + 4;
                let y = row + 2;
                match board.get(loc) {
                    board::Board::EMPTY => self.draw_text(x, y, Self::WHITE, ".")?,
                    board::Board::HUMAN => self.draw_text(x, y, Self::LT_RED, "⓿")?,
                    board::Board::COMPUTER => self.draw_text(x, y, Self::LT_BLUE, "⓿")?,
                    _ => panic!("Internal error in board state")
                }
            }
        }

        let human = format!("Human:    {}", board.get_score(board::Board::HUMAN).unwrap());
        self.draw_text(28, 2, Self::LT_RED, human.as_str())?;

        let computer = format!("Computer: {}", board.get_score(board::Board::COMPUTER).unwrap());
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

    // draw_valid_moves indicates the valid player moves on the screen.
    fn draw_valid_moves(&mut self, board: &board::Board) -> io::Result<()> {
        for row in 0..8 {
            for col in 0..8 {
                let loc = row*8 + col;
                let x = col*2 + 4;
                let y = row + 2;
                if (board.get(loc) == board::Board::EMPTY) && (board.count_move(loc, board::Board::HUMAN) > 0) {
                    self.draw_text(x, y, Self::RED, "?")?;
                }
            }
        }
        Ok(())
    }

    // goto_xy moves the cursor to the given 0-based (x, y) coordinates.
    fn goto_xy(&mut self, x: i32, y: i32) -> io::Result<()> {
        self.term.write_str(format!("\x1b[{};{}H", y+1, x+1).as_str())
    }

    // read_move reads a row (a-h) and column (1-8) from the user and
    // translates it into a location (0-63).  Only valid moves are accepted.
    pub fn read_move(&mut self, board: &board::Board) -> Option<i32> {
        loop {
            self.draw_valid_moves(board).unwrap_or(());
            self.draw_text(28, 8, Self::WHITE, "Row (a-h)? ").unwrap_or(());
            let mut row = -1;
            while row == -1 {
                let ch = self.term.read_char().expect("Terminal error");
                if ch == 'q' {
                    return None;
                } else if (ch >= 'a') && (ch <= 'h') {
                    row = (ch as i32) - ('a' as i32);
                    self.draw_text(39, 8, Self::LT_WHITE, format!("{}", ch).as_str()).unwrap_or(());
                }
            };

            let mut col = -1;
            self.draw_text(28, 9, Self::WHITE, "Col (1-8)? ").unwrap_or(());
            while col == -1 {
                let ch = self.term.read_char().expect("Terminal error");
                if ch == 'q' {
                    return None;
                } else if (ch >= '1') && (ch <= '8') {
                    col = (ch as i32) - ('1' as i32);
                    self.draw_text(39, 9, Self::LT_WHITE, format!("{}", ch).as_str()).unwrap_or(());
                }
            };

            let loc = row*8 + col;
            if board.count_move(loc, board::Board::HUMAN) > 0 {
                return Some(loc);
            }
            self.draw_text(28, 11, Self::LT_YELLOW, "Invalid move!").unwrap_or(());
            self.term.read_char().expect("Terminal error");
        }
    }

    // report_move informs the player of the computer's move.
    pub fn report_move(&mut self, loc: i32) {
        let text = format!("I moved to {}{}.", (((loc/8) as u8) + 97) as char, (loc%8) + 1);
        self.draw_text(28, 6, Self::LT_WHITE, text.as_str()).unwrap_or(());
    }

    // set_color sets the current terminal color to the one given.
    fn set_color(&mut self, color: u8) -> io::Result<()> {
        self.term.write_str(format!("\x1b[{}m", color).as_str())
    }

    // wait_for_key waits for the user to press a key, then discards it.
    pub fn wait_for_key(&mut self) {
        self.draw_text(28, 9, Self::LT_WHITE, "Press any key...").unwrap_or(());
        self.term.read_char().expect("Terminal error");
    }
}
