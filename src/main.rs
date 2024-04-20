use rand::Rng;
mod game_logic;
mod server;
use game_logic::Game;
use server::start_server;

fn place_bot_ship(bot_game: &mut Game) {
    let carrier: Vec<usize> = vec![0, 1, 0, 5];
    bot_game.place_ship(carrier);
    let destroyer: Vec<usize> = vec![4, 3, 7, 3];
    bot_game.place_ship(destroyer);
    let cruiser: Vec<usize> = vec![9, 3, 9, 6];
    bot_game.place_ship(cruiser);
    let submarine: Vec<usize> = vec![0, 7, 0, 9];
    bot_game.place_ship(submarine);
}

fn bot_turn(bot: &mut Game, player: &mut Game) -> bool {

    let mut rng = rand::thread_rng();
    let x: usize = rng.gen_range(0..10);
    let y: usize = rng.gen_range(0..10);
    bot.take_shot_from_coord(&mut player.ships, x, y);
    let state: bool = player.check_game_lost();
    state
}

fn main() {
    start_server()
}

