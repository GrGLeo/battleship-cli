use std::io;

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
            println!("Place your {}", ship);
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
        println!("Fire position");
        let mut pos = String::new();
        io::stdin().read_line(&mut pos).expect("Failed to read line");
        std::process::Command::new("clear").status().unwrap();
        self.take_shot(&mut bot.ships, pos);
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
        for row in &self.cells{
            for cell in row{
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

fn input_to_int(input: &str) -> Vec<usize> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let mut position: Vec<usize> = Vec::new();

    for part in parts{
        if let (Some::<char>(posy), Some::<char>(posx)) = (part.chars().next(), part.chars().nth(1)) {
            println!("{:?}", posy);
            let posy_usize = posy as usize - 'A' as usize;
            let posx_usize = posx.to_digit(10).unwrap();
            position.push(posy_usize);
            position.push(posx_usize.try_into().unwrap());
        }
    }
    position
}
