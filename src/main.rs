mod game_logic;
use game_logic::Game;
mod bot;
use bot::Bot;


fn main() {
    std::process::Command::new("clear").status().unwrap();
    let mut player = Game::new();
    let mut bot = Bot::new();
    bot.place_bot_ship();
    bot.game.display_both();
    player.place_ships();

    loop {
        let state = player.player_turn(&mut bot.game);
        if state {
            println!("You won!");
            break;
        }
        let state = bot.bot_turn(&mut player);
        if state {
            println!("You lost!");
            break;
        }
        std::process::Command::new("clear").status().unwrap();
        
    }
}

