# sudoku_solver
Sudoku Solver Written in Rust

## Usage
```rust
use sudoku_solver::{Sudoku, SudokuSolver};

fn main() {
    let sudoku = Sudoku::new(
        "800000000003600000070090200050007000000045700000100030001000068008500010090000400",
    );
    match sudoku {
        Ok(sudoku) => {
            println!("\n{}", sudoku);
            let mut sudoku_solver = SudokuSolver::new(sudoku);
            sudoku_solver.solve();
        }
        Err(e) => {
            eprintln!("{e}")
        }
    }
}
```
