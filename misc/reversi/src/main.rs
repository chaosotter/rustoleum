mod board;
mod screen;

fn main() {
    let mut board = board::Board::new();
    let mut screen = screen::Screen::new();

    let mut turn = board::Board::COMPUTER;
    let mut last_move = (-1, -1);

    while !board.game_over() {
        turn ^= 0b11; // 1 -> 2, 2 -> 1
        let turn_moves = board.get_moves(turn);
        if turn_moves.is_empty() {
            continue;
        }

        screen.draw_board(&board).unwrap_or(());
        if last_move.0 != -1 {
            screen.report_move(last_move.0, last_move.1).unwrap_or(());
        }

        if turn == board::Board::HUMAN {
            match screen.read_move(&board) {
                Some(move_) => board.do_move(move_.0, move_.1, board::Board::HUMAN),
                None => break,
            }
            screen.draw_board(&board).unwrap_or(());
            screen.wait_for_key();
        } else {
            last_move = board.select_move(turn_moves, turn);
            board.do_move(last_move.0, last_move.1, board::Board::COMPUTER);
        }
    }

    screen.draw_board(&board).unwrap_or(());
    screen.report_winner(&board).unwrap_or(());
}
