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

    fn take_shot(&mut self, board:&mut Board, pos:String) {
        let parts: Vec<&str> = pos.split(',').collect();

        if let [Ok(col), Ok(row)] = [
            parts[0].trim().parse::<usize>(),
            parts[1].trim().parse::<usize>()
        ]{
            if board.cells[row][col] == CellState::Ship {
                self.hits.cells[row][col] = CellState::Hit;
                board.cells[row][col] = CellState::Hit;
            }
            else {
                self.hits.cells[row][col] = CellState::Miss
            }
        }
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


fn main() {
    let mut player = Game::new();
    let mut bot = Game::new();
    
    player.ships.place_ship(2, 3);
    bot.ships.place_ship(2, 3);
    
    loop {
    println!("Fire position");
    let mut pos = String::new();
    io::stdin().read_line(&mut pos).expect("Failed to read line");
    player.take_shot(&mut bot.ships, pos);
    player.display_both();
    }

}
