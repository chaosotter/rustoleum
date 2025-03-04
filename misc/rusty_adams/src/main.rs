fn main() {
    let game = match rusty_adams::load_game("games/adv01.dat") {
        Ok(game) => game,
        Err(err) => panic!("{}", err.to_string()),
    };

    game.print_debug();
}
