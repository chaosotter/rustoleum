//! This module contains the Board type, which represents the state of the game
//! itself, with all I/O elements kept separate in the `screen.rs` module.

/// Board represents the state of the board.  For ease of coding, we use
/// a single-dimensional array of 64 elements, each of which is 0 for empty,
/// 1 for the human player, or 2 for the computer player.
///
/// We keep track of the score explicitly simply for the sake of efficiency.
pub struct Board {
    squares: [[u8; 8]; 8],
    scores: [i32; 2],
}

impl Board {
    pub const EMPTY: u8 = 0;
    pub const HUMAN: u8 = 1;
    pub const COMPUTER: u8 = 2;

    /// The deltas to apply to a row and column to move in some direction.
    const OFFSETS: [[i32; 2]; 8] = [
        [1, 0],   // right
        [1, -1],  // up & right
        [0, -1],  // up
        [-1, -1], // up & left
        [-1, 0],  // left
        [-1, 1],  // down & left
        [0, 1],   // down
        [1, 1],   // down & right
    ];

    /// The weights to assign to moves to each space on the board.  These
    /// values were assigned by rough experience in game play.
    const VALUES: [[i32; 8]; 8] = [
        [10, 2, 8, 6, 6, 8, 2, 10],
        [2, 1, 3, 4, 4, 3, 1, 2],
        [8, 3, 7, 5, 5, 7, 3, 8],
        [6, 4, 5, 1, 1, 5, 4, 6],
        [6, 4, 5, 1, 1, 5, 4, 6],
        [8, 3, 7, 5, 5, 7, 3, 8],
        [2, 1, 3, 4, 4, 3, 1, 2],
        [10, 2, 8, 6, 6, 8, 2, 10],
    ];

    /// Returns a new Board with the initial pieces placed.
    pub fn new() -> Self {
        let mut board = Self {
            squares: [[Self::EMPTY; 8]; 8],
            scores: [2, 2],
        };
        board.set(3, 3, Self::HUMAN);
        board.set(4, 4, Self::HUMAN);
        board.set(3, 4, Self::COMPUTER);
        board.set(4, 3, Self::COMPUTER);
        board
    }

    /// Counts the number of pieces that a player would flip in the given
    /// direction if they were to move to (col, row).  We assume that we have
    /// already checked that the space is empty.
    fn count_in_dir(&self, col: i32, row: i32, player: u8, dir: usize) -> i32 {
        let other = player ^ 0b11; // 1 -> 2, 2 -> 1
        let mut col = col;
        let mut row = row;
        let mut found = 0;

        loop {
            col += Self::OFFSETS[dir][0];
            if !(0..8).contains(&col) {
                return 0;
            }
            row += Self::OFFSETS[dir][1];
            if !(0..8).contains(&row) {
                return 0;
            }

            let val = self.get(col, row);
            if val == player {
                return found;
            } else if val == other {
                found += 1;
            } else {
                return 0;
            }
        }
    }

    /// Returns the number of pieces that a player would flip if they were to
    /// move to (col, row).  A return value of 0 implies an illegal move.
    pub fn count_move(&self, col: i32, row: i32, player: u8) -> i32 {
        if self.get(col, row) == Self::EMPTY {
            let mut count = 0;
            for dir in 0..8 {
                count += self.count_in_dir(col, row, player, dir);
            }
            count
        } else {
            0
        }
    }

    /// Makes the given move for the given player and adjusts the scores
    /// accordingly.  We assume that the move is already known to be valid.
    pub fn do_move(&mut self, col: i32, row: i32, player: u8) {
        self.set(col, row, player);
        self.scores[(player - 1) as usize] += 1;
        for dir in 0..8 {
            if self.count_in_dir(col, row, player, dir) > 0 {
                self.flip_dir(col, row, player, dir);
            }
        }
    }

    /// Flips the piece in the given square to that of the other player and
    /// adjusts the scores accordingly.
    fn flip(&mut self, col: i32, row: i32) {
        let old = self.get(col, row);
        let new = old ^ 0b11; // 1 -> 2, 2 -> 1
        self.set(col, row, new);
        self.scores[(old - 1) as usize] -= 1;
        self.scores[(new - 1) as usize] += 1;
    }

    /// Flips the pieces in the given direction to that of the other player
    /// and adjusts the scores accordingly.  We assume that the direction is
    /// already known to be valid.
    fn flip_dir(&mut self, col: i32, row: i32, player: u8, dir: usize) {
        let mut col = col;
        let mut row = row;
        loop {
            col += Self::OFFSETS[dir][0];
            row += Self::OFFSETS[dir][1];
            if self.get(col, row) == player {
                return;
            }
            self.flip(col, row);
        }
    }

    /// game_over checks whether the game is over, which means that neither
    /// player can make a valid move.  This condition is sufficient to detect
    /// both a full board and total defeat.
    pub fn game_over(&self) -> bool {
        self.get_moves(Self::HUMAN).is_empty() && self.get_moves(Self::COMPUTER).is_empty()
    }

    /// Returns the value of the square at the given location.
    pub fn get(&self, col: i32, row: i32) -> u8 {
        self.squares[row as usize][col as usize]
    }

    /// Returns all of the valid moves for the given player.
    pub fn get_moves(&self, player: u8) -> Vec<(i32, i32)> {
        let mut moves = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                if self.count_move(col, row, player) > 0 {
                    moves.push((col, row));
                }
            }
        }
        moves
    }

    /// Returns the score for the given player.
    pub fn get_score(&self, player: u8) -> Option<i32> {
        if player == Self::HUMAN || player == Self::COMPUTER {
            Some(self.scores[(player - 1) as usize])
        } else {
            None
        }
    }

    /// Sets the square at (col, row) to the given value.
    fn set(&mut self, col: i32, row: i32, value: u8) {
        self.squares[row as usize][col as usize] = value;
    }

    /// Picks a move from the given set of possible moves for the given player.
    ///
    /// Because this is a throwaway game written purely for the sake of learning
    /// Rust, we don't implement minimax or alpha-beta pruning or any of that
    /// good stuff, just an evaluation function based on position heuristics and
    /// the number of pieces flipped.
    pub fn select_move(&self, moves: Vec<(i32, i32)>, player: u8) -> (i32, i32) {
        let mut best = moves[0];
        let mut best_score = -1;
        for move_ in moves {
            let mult = Self::VALUES[move_.1 as usize][move_.0 as usize];
            let score = self.count_move(move_.0, move_.1, player) * mult;
            if score > best_score {
                best = move_;
                best_score = score;
            }
        }
        best
    }
}
