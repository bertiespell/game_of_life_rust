use std::fmt::Display;
use std::fmt;

fn main() {
    let new_board = initialise_gameboard();
    println!("{}", new_board);
}

#[derive(Copy, Clone)]
enum CellState {
    Alive,
    Dead
}

// 16 * 16
struct Board { 
    cells: Vec<Vec<CellState>>
}

impl Board {
    fn new() -> Board {
        let newboard = Board {
            cells: vec![vec![CellState::Alive; 16]; 16]
        };
        newboard
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.cells.iter().map(|row| {
            row.iter().map(|cell| fmt::write(f, format_args!("{}", cell))).collect::<Vec<_>>();
            println!();
        }).collect::<Vec<_>>();
        Ok(())
    }
}

impl Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let match_result = match self {
            CellState::Alive => "A",
            CellState::Dead => "D"
        };
        fmt::write(f, format_args!("{}", match_result))
    }
}

fn initialise_gameboard() -> Board {
    Board::new()
}