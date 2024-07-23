use crate::grid::Grid;
use crate::solver::Solver;

pub struct BruteForceSolver;

impl BruteForceSolver {
    pub fn new() -> Self {
        BruteForceSolver
    }
}

impl Solver for BruteForceSolver {
    fn solve(&mut self, input: &Grid) -> Option<Grid> {
        let mut grid = input.clone();
        let empty_cells = input.get_empty_cells();

        let mut i = 0;
        loop {
            let (row, col) = empty_cells[i];
            let mut value = grid.get(row, col);

            if value < 9 {
                value += 1;
                grid.set(row, col, value);
                if grid.is_valid() {
                    if grid.is_solved() {
                        break; // solution found
                    }
                    i += 1;
                }
            } else {
                if i == 0 {
                    return None; // no solution
                }
                grid.unset(row, col);
                i -= 1;
            }
        }

        Some(grid)
    }
}
