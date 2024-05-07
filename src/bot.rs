use rand::Rng;
use rand::seq::SliceRandom;
use crate::{Game,CellState};
use std::collections::VecDeque;
use crate::utils::logger;

pub struct Bot {
    pub game: Game,
    hits: Vec<(usize, usize)>,
    last_hit: VecDeque<CellState>,
    last_ship_pos: VecDeque<(usize, usize)>, 
    searching: bool,
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            game: Game::new(),
            hits: Vec::new(),
            last_hit: VecDeque::with_capacity(3),
            last_ship_pos: VecDeque::with_capacity(3), 
            searching: true,
        }
    }

    // Ship placement
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

    // Shooting configuration
    pub fn hit_tracker(&mut self, celltype: CellState, row: usize, col: usize) {
        self.hits.push((row, col));
        self.last_hit.push_back(celltype.clone());
        if self.last_hit.len() > 3 {
            self.last_hit.pop_front();
        }
        if celltype == CellState::Hit {
            self.searching = false;

            if self.last_ship_pos.len() == 0 {
                self.last_ship_pos.push_back((row, col));
            }
            else {
                if self.last_ship_pos[0].0 > row || self.last_ship_pos[0].1 > col {
                    self.last_ship_pos.push_front((row, col));
                }
                else {
                    self.last_ship_pos.push_back((row, col));
                }
            }
        }
        else if self.last_hit.iter().all(|state| state == &CellState::Miss) {
            self.searching = true;
            self.last_ship_pos = VecDeque::with_capacity(5);
        }
        
    }

    pub fn detect_axis(&self) -> bool {
        // return true if ship is horizontal
        self.last_ship_pos[0].0 == self.last_ship_pos[1].0 
    }

    fn predict_ship_coordinate(&mut self) -> (usize, usize) {
        let mut attempt: u8 = 0;
        let h = self.detect_axis();
        let length = self.last_ship_pos.len() - 1;
        if h {
            let possible_coord = vec! [
                (self.last_ship_pos[0].0, self.last_ship_pos[0].1.saturating_sub(1)),
                (self.last_ship_pos[0].0, self.last_ship_pos[length].1.saturating_sub(1))
            ];
            loop {
                if attempt > 5 {
                    self.searching = true;
                    self.last_ship_pos = VecDeque::with_capacity(5);
                    return self.random_shoot()
                }
                if let Some(coord) = possible_coord.choose(&mut rand::thread_rng()) {
                    if coord.0 < 10 && coord.1 < 10 && !self.hits.contains(coord) {
                        return *coord
                    } else {
                        attempt += 1;
                        continue
                    }
                }

            }
        }
        else {
            let possible_coord = vec! [
                (self.last_ship_pos[0].0.saturating_sub(1), self.last_ship_pos[0].1),
                (self.last_ship_pos[length].0.saturating_add(1), self.last_ship_pos[length].1)
            ];
            loop {
                if attempt > 5 {
                    self.searching = true;
                    self.last_ship_pos = VecDeque::with_capacity(5);
                    return self.random_shoot()
                }
                if let Some(coord) = possible_coord.choose(&mut rand::thread_rng()) {
                    if coord.0 < 10 && coord.1 < 10 && !self.hits.contains(coord) {
                        return *coord
                    } else {
                        attempt += 1;
                        continue
                    }
                }
            }
        }
    }

    fn target_around_ship(&mut self) -> (usize, usize) {
        let mut attempt: u8 = 0;
        let (last_row, last_col) = self.last_ship_pos[0];
        let possible_coord = vec![
            (last_row.saturating_sub(1), last_col),
            (last_row.saturating_add(1), last_col),
            (last_row, last_col.saturating_sub(1)),
            (last_row, last_col.saturating_add(1))
        ];
        loop {
            if attempt > 8 {
                self.searching = false;
                self.last_ship_pos = VecDeque::with_capacity(5);
                return self.random_shoot()
            }
            if let Some(coord) = possible_coord.choose(&mut rand::thread_rng()) {
                if coord.0 < 10 && coord.1 < 10 && !self.hits.contains(coord) {
                    return *coord
                } else {
                    attempt += 1;
                    continue
                }
            }
        }
    }
    
    fn random_shoot(&mut self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        loop {
            let row:  usize = rng.gen_range(0..10);
            let col: usize = rng.gen_range(0..10);
            if !self.hits.contains(&(row, col)) {
                return (row, col)
            }
        }
    }

    fn target_shoot(&mut self) -> (usize, usize) {
        if self.last_ship_pos.len() < 2 {
            self.target_around_ship()
        }
        else {
            self.predict_ship_coordinate()
        }
    }

    pub fn shoot(&mut self, player: &mut Game) {
        let (row, col);

        if self.searching {
            let coord = self.random_shoot();
            row = coord.0;
            col = coord.1;
        }
        else {
            let coord = self.target_shoot();
            row = coord.0;
            col = coord.1;
        }

        self.game.take_shot_from_coord(&mut player.ships, col, row);
        let celltype = &self.game.hits.cells[row][col];
        self.hit_tracker(celltype.clone(), row, col);
        logger("bot", &(row, col))
    }

    pub fn bot_turn(&mut self, player: &mut Game) -> bool {
        self.shoot(player);
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
