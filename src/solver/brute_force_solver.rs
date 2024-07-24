use crate::solver::Solver;
use crate::Sudoku;

pub struct BruteForceSolver;

impl BruteForceSolver {
    pub fn new() -> Self {
        BruteForceSolver
    }
}

impl Solver for BruteForceSolver {
    fn solve(&mut self, input: &Sudoku) -> Option<String> {
        let mut sudoku = input.clone();
        let empty_cells = input.get_empty_cells();

        let mut i = 0;
        loop {
            let (row, col) = empty_cells[i];
            let mut value = sudoku.get(row, col);

            if value < 9 {
                value += 1;
                sudoku.set(row, col, value);
                if sudoku.is_valid() {
                    if sudoku.is_solved() {
                        break; // solution found
                    }
                    i += 1;
                }
            } else {
                if i == 0 {
                    return None; // no solution
                }
                sudoku.unset(row, col);
                i -= 1;
            }
        }

        Some(sudoku.to_string())
    }
}
