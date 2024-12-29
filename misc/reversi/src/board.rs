//! This module contains the Board type, which represents the state of the game
//! itself, with all I/O elements kept separate in the `screen.rs` module.

/// Board represents the state of the board.  For ease of coding, we use
/// a single-dimensional array of 64 elements, each of which is 0 for empty,
/// 1 for the human player, or 2 for the computer player.
///
/// We keep track of the score explicitly simply for the sake of efficiency.
pub struct Board {
    squares: [u8; 64],
    scores: [i32; 2]
}

impl Board {
    pub const EMPTY: u8 = 0;
    pub const HUMAN: u8 = 1;
    pub const COMPUTER: u8 = 2;
    
    const OFFSETS: [i32; 8] = [ 1, -7, -8, -9, -1, 7, 8, 9 ];
    
    const VALUES: [i32; 64] = [
        10, 2, 8, 5, 5, 8, 2, 10,
         2, 1, 3, 4, 4, 3, 1,  2,
         8, 3, 5, 3, 3, 5, 3,  8,
         5, 4, 3, 1, 1, 3, 4,  5,
         5, 4, 3, 1, 1, 3, 4,  5,
         8, 3, 5, 3, 3, 5, 3,  8,
         2, 1, 3, 4, 4, 3, 1,  2,
        10, 2, 8, 5, 5, 8, 2, 10
    ];

    /// new returns a new Board with the initial pieces placed.
    pub fn new() -> Self {
        let mut board = Self {
            squares: [Self::EMPTY; 64],
            scores: [2, 2]
        };
        board.set(27, Self::HUMAN);
        board.set(36, Self::HUMAN);
        board.set(28, Self::COMPUTER);
        board.set(35, Self::COMPUTER);
        board
    }

    // count_in_dir counts the number of pieces that a player would flip in the
    // given direction if they were to move to |loc|.  We assume that we have
    // already checked that the space is empty.
    fn count_in_dir(&self, loc: i32, player: u8, dir: i32) -> i32 {
        let other = player ^ 0b11;  // 1 -> 2, 2 -> 1
        let mut this = loc;
        let mut found = 0;
        loop {
            this += Self::OFFSETS[dir as usize];
            if (this < 0) || (this >= 64) {
                return 0;
            }
            let val = self.squares[this as usize];
            if val == player {
                return found;
            } else if val == other {
                found += 1;
            } else {
                return 0;
            }
        }
    }

    // count_move returns the number of pieces that a player would flip if they
    // were to move to |loc|.  A return value of 0 implies an illegal move.
    pub fn count_move(&self, loc: i32, player: u8) -> i32 {
        if self.get(loc) == Self::EMPTY {
            let mut count = 0;
            for dir in 0..8 {
                count += self.count_in_dir(loc, player, dir);
            }
            count
        } else {
            0
        }
    }

    // do_move makes the given move for the given player and adjusts the scores
    // accordingly.  We assume that the move is already known to be valid.
    pub fn do_move(&mut self, loc: i32, player: u8) {
        self.set(loc, player);
        self.scores[(player-1) as usize] += 1;
        for dir in 0..8 {
            if self.count_in_dir(loc, player, dir) > 0 {
                self.flip_dir(loc, player, dir);
            }
        }
    }

    // flip flips the piece in the given square to that of the other player and
    // adjusts the scores accordingly.
    fn flip(&mut self, loc: i32) {
        let old = self.get(loc);
        let new = old ^ 0b11;  // 1 -> 2, 2 -> 1
        self.set(loc, new);
        self.scores[(old-1) as usize] -= 1;
        self.scores[(new-1) as usize] += 1;
    }

    // flip_dir flips the pieces in the given direction to that of the other
    // player and adjusts the scores accordingly.  We assume that the direction
    // is already known to be valid.
    fn flip_dir(&mut self, loc: i32, player: u8, dir: i32) {
        let mut this = loc;
        loop {
            this += Self::OFFSETS[dir as usize];
            let val = self.squares[this as usize];
            if val == player {
                return;
            }
            self.flip(this);
        }
    }

    /// game_over checks whether the game is over, which means that neither
    /// player can make a valid move.  This condition is sufficient to detect
    /// both a full board and total defeat.
    pub fn game_over(&self) -> bool {
        self.get_moves(Self::HUMAN).is_empty()
            && self.get_moves(Self::COMPUTER).is_empty()
    }

    // get returns the value of the square at the given location.
    pub fn get(&self, loc: i32) -> u8 {
        self.squares[loc as usize]
    }

    // get_moves returns all of the valid moves for the given player.
    pub fn get_moves(&self, player: u8) -> Vec<i32> {
        let mut moves = Vec::new();
        for loc in 0..64 {
            if self.count_move(loc, player) > 0 {
                moves.push(loc);
            }
        }
        moves
    }

    /// get_score returns the score for the given player.
    pub fn get_score(&self, player: u8) -> Option<i32> {
        if player == Self::HUMAN || player == Self::COMPUTER {
            Some(self.scores[(player-1) as usize])
        } else {
            None
        }
    }

    // set sets the square at |loc| to the given value.
    fn set(&mut self, loc: i32, value: u8) {
        self.squares[loc as usize] = value;
    }

    // select_move picks a move from the given set of possible moves for the
    // given player.
    //
    // Because this is a throwaway game written purely for the sake of learning
    // Rust, we don't implement minimax or alpha-beta pruning or any of that
    // good stuff, just an evaluation function based on position heuristics and
    // the number of pieces flipped.
    pub fn select_move(&self, moves: Vec<i32>, player: u8) -> i32 {
        let mut best = moves[0];
        let mut best_score = -1;
        for move_ in moves {
            let score = self.count_move(move_, player) * Self::VALUES[move_ as usize];
            if score > best_score {
                best = move_;
                best_score = score;
            }
        }
        best
    }
}
