#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    X,
    O,
}

#[derive(Debug)]
struct Board {
    cells: [Cell; 9],
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [Cell::Empty; 9],
        }
    }

    fn display(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            match cell {
                Cell::Empty => print!("   "),
                Cell::X => print!(" X "),
                Cell::O => print!(" O "),
            }

            if i % 3 == 2 {
                println!();
                if i != 8 {
                    println!("-----------");
                }
            } else {
                print!("|");
            }
        }
    }
}

fn main() {
    let board = Board::new();
    board.display();
}
