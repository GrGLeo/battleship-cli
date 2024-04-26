use rand::Rng;
use crate::Game;

pub struct Bot {
    pub game: Game,
    hits: Vec<(usize, usize)>,
    last_hit: Option<(usize, usize)>,
    searching: bool,
    turn: u8
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            game: Game::new(),
            hits: Vec::new(),
            last_hit: None,
            searching: true,
            turn: 0
        }
    }

    pub fn track_hit(&mut self, row: usize, col: usize) {
        self.hits.push((row, col));
        self.last_hit = Some((row, col));
    }

    pub fn place_bot_ship(&mut self) {
        let sizes: Vec<i32> = vec![5, 4, 3, 3];
        for size in sizes {
            loop {
                let mut ship_size = size.clone();
                let position = self.get_position(ship_size);
                let placed: bool = self.game.place_pos(position, &mut ship_size, false);
                if placed {
                    break
                }
            }
        }
    }

    pub fn get_position(&mut self, size: i32) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let max: usize = 10 - size as usize;
        let horizontal: bool = rng.gen();
        if horizontal {
            let x: usize = rng.gen_range(0..10);
            let y1: usize = rng.gen_range(0..max);
            let y2: usize = y1 + size as usize - 1;
            let position: Vec<usize> =  vec![x, y1, x, y2];
            position
        }
        else {
            let y: usize = rng.gen_range(0..10);
            let x1: usize = rng.gen_range(0..max);
            let x2: usize = x1 + size as usize - 1;
            let position: Vec<usize> =  vec![x1, y, x2, y];
            position
        }
    }

    pub fn shoot(&mut self, player: &mut Game) {
        if self.searching {
            let mut rng = rand::thread_rng();
            let x: usize = rng.gen_range(0..10);
            let y: usize = rng.gen_range(0..10);
            self.game.take_shot_from_coord(&mut player.ships, x, y);
            self.track_hit(y, x)
        }
    }

    pub fn bot_turn(&mut self, player: &mut Game) -> bool {
        self.shoot(player);
        self.turn += 1;
        player.check_game_lost()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::game_logic::CellState;
    
    #[test]
    fn test_bot_ship_placement(){
        let mut bot = Bot::new();
        bot.place_bot_ship();
        let ship_count = bot.game.ships.cells.iter()
            .flatten()
            .filter(|&cell| *cell == CellState::Ship)
            .count(); 
        assert_eq!(ship_count, 15)
    }
}
