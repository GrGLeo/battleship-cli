mod utils;
use utils::{input_to_int, reorder_position, read_input};

#[derive(Debug, Clone, PartialEq)]
pub enum CellState {
    Empty,
    Ship,
    Hit,
    Miss,
}

pub struct Game {
    pub ships: Board,
    hits: Board,
}

impl Game {
    pub fn new() -> Self {
        let ships = Board::new();
        let hits = Board::new();
        Game{ships, hits}
    }

    pub fn display_both(&self) {
        self.hits.display();
        println!();
        self.ships.display();
    }

    pub fn place_ships(&mut self) {
        let ships = ["Carrier", "Destroyer", "Cruiser", "Submarine"];
        let sizes = [5, 4, 3, 3];
        for i in 0..ships.len(){
            let mut placed: bool = false;
            while !placed {
                let mut size = sizes[i].clone();
                placed = self.place_ship(&ships[i], &mut size);
            }
        }
    } 
    
     pub fn place_ship(&mut self, ship: &str, size: &mut i32) -> bool {
         self.ships.display();
         println!("Place your {} (y1x1 y2x2)", ship);
         let input_type = "ship";
         let input: String = read_input(&input_type);
         std::process::Command::new("clear").status().unwrap();
         if input.trim().len() != 5 {
             return false
         }
         let mut position = input_to_int(&input);
         reorder_position(&mut position);
         let placed = self.place_pos(position, size, true);
         placed
     }

     pub fn place_pos(&mut self, position: Vec<usize>, size: &mut i32, verbose: bool) -> bool{
         let mut visited_pos: Vec<Vec<usize>> = Vec::new();
         for row in position[0]..=position[2]{
             for cell in position[1]..=position[3]{
                 match self.ships.cells[row][cell] {
                     CellState::Empty => {
                         self.ships.cells[row][cell] = CellState::Ship;
                         visited_pos.push(vec![row, cell]);
                         *size -= 1;
                     },
                     CellState::Ship => {
                         break 
                     },
                     _ => {},
                 }
             }
         }
         if *size == 0 {
             return true
         } 
         else {
             if verbose {
             println!("Error during placement, please provide correct coordinate.");
             }
             for pos in visited_pos {
                 self.ships.cells[pos[0]][pos[1]] = CellState::Empty;
             }
             return false
         }
     }
            

     pub fn take_shot(&mut self, board:&mut Board, pos:String) -> bool {
         if pos.trim().len() != 2 {
             println!("Wrong coordinate!");
             return false
         } 
         if let (Some::<char>(posy), Some::<char>(posx)) = (pos.chars().next(), pos.chars().nth(1)) {
             let row = posy as usize - 'A' as usize;
             let col = posx.to_digit(10).unwrap() as usize;
             match board.cells[row][col] {
                 CellState::Ship => {
                     self.hits.cells[row][col] = CellState::Hit;
                     board.cells[row][col] = CellState::Hit;
                 },
                 CellState::Miss | CellState::Hit => {
                     println!("Position already fired!");
                     return false
                 },
                 _ => {
                     self.hits.cells[row][col] = CellState::Miss;
                     board.cells[row][col] = CellState::Miss;
                 }
             }
         }
         return true
     }

    pub fn take_shot_from_coord(&mut self, board: &mut Board, col: usize, row: usize) {
        if board.cells[row][col] == CellState::Ship {
            self.hits.cells[row][col] = CellState::Hit;
            board.cells[row][col] = CellState::Hit;
        }
        else {
            self.hits.cells[row][col] = CellState::Miss;
            board.cells[row][col] = CellState::Miss;
        }
    }
    
    pub fn check_game_lost(&self) -> bool {
        let ship_count = self.ships.cells.iter().flatten().filter(|&cell| *cell == CellState::Ship).count();
        if ship_count == 0{
            true
        }
        else {false}
    }

    pub fn player_turn(&mut self, other: &mut Game) -> bool {
        self.display_both();
        loop {
            println!("Fire position (yx)");
            let input_type = "fire";
            let pos = read_input(&input_type);
            let shoot = self.take_shot(&mut other.ships, pos);
            if shoot { break };
        }
        std::process::Command::new("clear").status().unwrap();
        let state: bool = other.check_game_lost();
        state
    }
}

pub struct Board {
    pub cells: Vec<Vec<CellState>>,
}

impl Board {
    fn new() -> Self {
        let cells = vec![vec![CellState::Empty; 10]; 10];
        Board { cells }
    }
    

    fn display(&self) {
            print!("  "); // Empty space for alignment
            for col_label in 0..10 {
                print!("{} ", col_label);
            }
            println!(); 

            for (row_index, row) in self.cells.iter().enumerate() {
                let ascii_code: u32 = <usize as TryInto<u32>>::try_into(row_index).unwrap() + 65;
                let ascii = std::char::from_u32(ascii_code).unwrap();
                print!("{} ",ascii);
                for cell in row {
                    match cell {
                        CellState::Empty => print!(". "),
                        CellState::Ship => print!("S "),
                        CellState::Hit => print!("X "),
                        CellState::Miss => print!("0 "),
                    }
                }
                println!();
            }
        }
}

#[cfg(test)]
mod test {
    use super::*;
}
