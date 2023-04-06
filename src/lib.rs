pub struct Sudoku {
    cells: [u8; 81],
}

impl Sudoku {
    pub fn new(input: &str) -> Result<Self, String> {
        let mut cells = [0; 81];
        if input.len() != 81 {
            return Err(String::from("ERROR: Input strings length needs to be 81."));
        }

        for (i, c) in input.chars().enumerate() {
            if !c.is_ascii_digit() {
                return Err(String::from(format!("ERROR: {c} is not a valid string.")));
            }

            let a = c.to_digit(10).unwrap() as u8;
            cells[i] = a;
        }

        Ok(Self { cells })
    }
}

pub struct SudokuSolver {
    sudoku: Sudoku,
}

impl SudokuSolver {
    pub fn new(sudoku: Sudoku) -> Self {
        Self { sudoku }
    }

    fn is_safe(&self, row: usize, col: usize, num: u8) -> bool {
        for x in 0..9 {
            if self.sudoku.cells[col * 9 + x] == num {
                return false;
            }
        }

        for x in 0..9 {
            if self.sudoku.cells[x * 9 + row] == num {
                return false;
            }
        }

        let sr = row - row % 3;
        let sc = col - col % 3;

        for y in 0..3 {
            for x in 0..3 {
                if self.sudoku.cells[(y + sc) * 9 + x + sr] == num {
                    return false;
                }
            }
        }

        return true;
    }
    pub fn solve(&mut self) {
        self._solve(0, 0);
        println!(" ANSWER:\n\n{}", self.sudoku);
    }
    fn _solve(&mut self, row: usize, col: usize) -> bool {
        if row == 9 {
            return true;
        } else if col == 9 {
            return self._solve(row + 1, 0);
        } else if self.sudoku.cells[col * 9 + row] != 0 {
            return self._solve(row, col + 1);
        } else {
            for num in 1..10 {
                if self.is_safe(row, col, num) {
                    self.sudoku.cells[col * 9 + row] = num;
                    if self._solve(row, col + 1) {
                        return true;
                    }

                    self.sudoku.cells[col * 9 + row] = 0;
                }
            }

            return false;
        }
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..9 {
            for x in 0..9 {
                if x == 3 || x == 6 {
                    write!(f, "|")?;
                }
                write!(f, " {} ", self.cells[y * 9 + x])?;
            }
            if y == 2 || y == 5 {
                write!(f, "\n ---------------------------\n")?;
            } else {
                write!(f, "\n\n")?;
            }
        }

        Ok(())
    }
}
