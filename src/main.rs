use std::io;

#[derive(Clone, Copy, PartialEq,Debug)]

enum Cell {
    Empty,
    X,
    O,
}

struct Board {
    cells: [Cell; 9],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [Cell::Empty; 9],
        }
    }

    fn print(&self) {
        for i in 0..3 {
            for j in 0..3 {
                match self.cells[i * 3 + j] {
                    Cell::Empty => print!("_ "),
                    Cell::X => print!("X "),
                    Cell::O => print!("O "),
                }
            }
            println!();
        }
    }
    fn make_move(&mut self, position: usize, player: Cell) -> bool {
        if position < 9 && self.cells[position] == Cell::Empty {
            self.cells[position] = player;
            true
        } else {
            false
        }
    }

    fn is_winner(&self, player: Cell) -> bool {
        let winning_combinations = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
            [0, 4, 8], [2, 4, 6],            // Diagonals
        ];

        winning_combinations.iter().any(|&[a, b, c]| {
            self.cells[a] == player && self.cells[b] == player && self.cells[c] == player
        })
    }

    fn is_full(&self) -> bool {
        !self.cells.contains(&Cell::Empty)
    }

}

fn get_player_move() -> usize {
    loop {
        println!("Enter your move (0-8):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) if num < 9 => return num,
            _ => println!("Please enter a valid number between 0 and 8."),
        }
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = Cell::X;

    loop {
        println!("\nCurrent board:");
        board.print();

        let position = get_player_move();
        if board.make_move(position, current_player) {
            if board.is_winner(current_player) {
                println!("\nPlayer {:?} wins!", current_player);
                board.print();
                break;
            } else if board.is_full() {
                println!("\nIt's a draw!");
                board.print();
                break;
            }
            current_player = if current_player == Cell::X { Cell::O } else { Cell::X };
        } else {
            println!("Invalid move, try again.");
        }
    }
}