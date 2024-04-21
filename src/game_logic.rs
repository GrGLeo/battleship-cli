use std::io;
mod utils;
use utils::input_to_int;

#[derive(Debug, Clone, PartialEq)]
enum CellState {
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
        let ships = ["Carrier","Destroyer",  "Cruiser", "Submarine"];
        for ship in ships {
            self.ships.display();
            let mut input = String::new();
            println!("Place your {} (y1x1 y2x2)", ship);
            io::stdin().read_line(&mut input).expect("Failed to read line");
            std::process::Command::new("clear").status().unwrap();
            let position = input_to_int(&input);
            self.place_ship(position);
        }
    } 

     pub fn place_ship(&mut self, position: Vec<usize>){
        for row in position[0]..=position[2]{
            for cell in position[1]..=position[3]{
                self.ships.cells[row][cell] = CellState::Ship;
            }
        }
    }
            

    pub fn take_shot(&mut self, board:&mut Board, pos:String) {
        
       if let (Some::<char>(posy), Some::<char>(posx)) = (pos.chars().next(), pos.chars().nth(1)) {
            let row = posy as usize - 'A' as usize;
            let col = posx.to_digit(10).unwrap() as usize;

                if board.cells[row][col] == CellState::Ship {
                    self.hits.cells[row][col] = CellState::Hit;
                    board.cells[row][col] = CellState::Hit;
                }
                else {
                    self.hits.cells[row][col] = CellState::Miss
                }
            }
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

    pub fn player_turn(&mut self, bot: &mut Game) -> bool {
        self.display_both();
        println!("Fire position (yx)");
        let mut pos = String::new();
        io::stdin().read_line(&mut pos).expect("Failed to read line");
        self.take_shot(&mut bot.ships, pos);
        std::process::Command::new("clear").status().unwrap();
        let state: bool = bot.check_game_lost();
        state
    }
}

pub struct Board {
    cells: Vec<Vec<CellState>>,
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

