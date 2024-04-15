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

    fn place_ships(&mut self) {
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

    fn place_ship(&mut self, position: Vec<usize>){
        for row in position[0]..=position[2]{
            for cell in position[1]..=position[3]{
                self.ships.cells[row][cell] = CellState::Ship;
            }
        }
    }
            

    fn take_shot(&mut self, board:&mut Board, pos:String) {
        
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
            let posx_usize = posx.to_digit(10).unwrap();
            position.push(posy_usize);
            position.push(posx_usize.try_into().unwrap());
        }
    }
    position
}

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

fn main() {
    let mut player = Game::new();
    let mut bot = Game::new();
    
    player.place_ships();
    place_bot_ship(&mut bot);
    loop {
        println!("Fire position");
        let mut pos = String::new();
        io::stdin().read_line(&mut pos).expect("Failed to read line");
        std::process::Command::new("clear").status().unwrap();
        player.take_shot(&mut bot.ships, pos);
        player.display_both();
        let state: bool = bot.check_game_lost();
        if state {
            println!("You won!");
            break
        } 
    }

}
