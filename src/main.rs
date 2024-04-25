mod game_logic;
use game_logic::Game;
mod bot;
use bot::Bot;
mod utils;

fn main() {
    let mut start_again = true;
    while start_again {
        std::process::Command::new("clear").status().unwrap();
        let mut player = Game::new();
        let mut bot = Bot::new();
        bot.place_bot_ship();
        player.place_ships();

        loop {
            let state = player.player_turn(&mut bot.game);
            if state {
                utils::print_win();
                start_again = utils::start_again();
                break;
            }
            let state = bot.bot_turn(&mut player);
            if state {
                println!("You lost!");
                break;
            }
            std::process::Command::new("clear").status().unwrap();

        }
        std::process::Command::new("clear").status().unwrap();
    }
}

