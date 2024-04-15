use std::io;

#[derive(Debug, Clone, PartialEq)]
enum CellState {
    Empty,
    Ship,
    Hit,
    Miss,
}

struct Game {
    ships: Board,
    hits: Board,
}

impl Game {
    fn new() -> Self {
        let ships = Board::new();
        let hits = Board::new();
        Game{ships, hits}
    }

    fn display_both(&self) {
        self.hits.display();
        println!();
        self.ships.display();
    }

    fn place_ship(&mut self) {
        let ships = ["Carrier","Destroyer",  "Cruiser", "Submarine"];
        for ship in ships {
            let mut input = String::new();
            println!("Place your {}", ship);
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let position = input_to_int(&input);

            for row in position[0]..=position[2]{
                for cell in position[1]..=position[3]{
                    self.ships.cells[row][cell] = CellState::Ship;
                }
            }
            self.ships.display()
        }
    } 
            

    fn take_shot(&mut self, board:&mut Board, pos:String) {
        
       if let (Some::<char>(posy), Some::<char>(posx)) = (pos.chars().next(), pos.chars().nth(1)) {
            let row = posy as usize - 'A' as usize;
            let col = posx.to_digit(10).unwrap() as usize -1 ;

                if board.cells[row][col] == CellState::Ship {
                    self.hits.cells[row][col] = CellState::Hit;
                    board.cells[row][col] = CellState::Hit;
                }
                else {
                    self.hits.cells[row][col] = CellState::Miss
                }
            }
    }
    
    fn check_game_lost(&self) -> bool {
        let ship_count = self.ships.cells.iter().flatten().filter(|&cell| *cell == CellState::Ship).count();
        if ship_count == 0{
            true
        }
        else {false}
    }
}

struct Board {
    cells: Vec<Vec<CellState>>,
}

impl Board {
    fn new() -> Self {
        let cells = vec![vec![CellState::Empty; 10]; 10];
        Board { cells }
    }
    
    fn place_ship(&mut self, row: usize, col: usize) {
        self.cells[row][col] = CellState::Ship
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
            let posy_usize = posy as usize - 'A' as usize;
            let posx_usize = posx.to_digit(10).unwrap() -1;
            position.push(posy_usize);
            position.push(posx_usize.try_into().unwrap());
        }
    }
    position
}

fn main() {
    let mut player = Game::new();
    let mut bot = Game::new();
    
    player.place_ship();
     
    bot.ships.place_ship(2, 3);
    loop {
        println!("Fire position");
        let mut pos = String::new();
        io::stdin().read_line(&mut pos).expect("Failed to read line");
        player.take_shot(&mut bot.ships, pos);
        player.display_both();
        let state: bool = bot.check_game_lost();
        if state {
            println!("You won!");
            break
        } 
    }

}
