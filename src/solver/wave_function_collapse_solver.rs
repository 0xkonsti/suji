use super::Solver;
use crate::Sudoku;

const BOXES: [[usize; 9]; 9] = [
    [0, 0, 0, 1, 1, 1, 2, 2, 2],
    [0, 0, 0, 1, 1, 1, 2, 2, 2],
    [0, 0, 0, 1, 1, 1, 2, 2, 2],
    [3, 3, 3, 4, 4, 4, 5, 5, 5],
    [3, 3, 3, 4, 4, 4, 5, 5, 5],
    [3, 3, 3, 4, 4, 4, 5, 5, 5],
    [6, 6, 6, 7, 7, 7, 8, 8, 8],
    [6, 6, 6, 7, 7, 7, 8, 8, 8],
    [6, 6, 6, 7, 7, 7, 8, 8, 8],
];

const BOX_TO_CELLS: [[usize; 9]; 9] = [
    [0, 1, 2, 9, 10, 11, 18, 19, 20],
    [3, 4, 5, 12, 13, 14, 21, 22, 23],
    [6, 7, 8, 15, 16, 17, 24, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],
];

pub struct WaveFunctionCollapseSolver {
    permutations: [u16; 81],
}

impl WaveFunctionCollapseSolver {
    pub fn new() -> Self {
        WaveFunctionCollapseSolver {
            permutations: [0; 81],
        }
    }

    fn generate_permutations(&mut self, sudoku: &Sudoku) {
        for i in 0..81 {
            self.permutations[i] = sudoku.get_possible_values(i / 9, i % 9);
        }
    }

    // Reduce the permutations by performin further logic
    //
    // 1. If a cell is the only one in a row, column or box that can contain a value, then that
    //    cell must contain that value.
    fn logic_process(&mut self, sudoku: &Sudoku) {
        // TODO: Implement this
    }

    fn next_best_cell(&self, sudoku: &Sudoku) -> Option<(usize, Vec<u8>)> {
        let mut best_cell = None;
        let mut best_count = 10u8;
        let mut best_values = Vec::new();
        for i in 0..81 {
            let count = self.permutations[i].count_ones() as u8;
            if count == 0 {
                if sudoku.get(i / 9, i % 9) != 0 {
                    continue;
                }
                return None;
            }
            if count == 1 {
                return Some((i, vec![self.permutations[i].trailing_zeros() as u8 + 1]));
            }
            if count < best_count {
                best_count = count;
                best_cell = Some(i);
                best_values = (1..=9).filter(|&v| self.permutations[i] & 1 << (v - 1) != 0).collect();
            }
        }

        best_cell.map(|cell| (cell, best_values))
    }

    fn collapse(&mut self, cell: usize, value: u8) {
        self.permutations[cell] = 0;
        let mask = !(1 << (value - 1));
        for i in 0..9 {
            let row_idx = cell / 9;
            let col_idx = cell % 9;
            let box_idx = BOXES[row_idx][col_idx];
            self.permutations[row_idx * 9 + i] &= mask;
            self.permutations[i * 9 + col_idx] &= mask;
            self.permutations[BOX_TO_CELLS[box_idx][i]] &= mask;
        }
    }

    fn solve_recursive(&mut self, sudoku: &Sudoku) -> Option<String> {
        let (cell, values) = match self.next_best_cell(sudoku) {
            Some(result) => result,
            None => {
                if sudoku.is_solved() {
                    return Some(sudoku.to_string());
                }
                return None;
            }
        };

        let (row, col) = (cell / 9, cell % 9);
        for value in values {
            let mut new_sudoku = sudoku.clone();
            let perm_state = self.permutations.clone();
            new_sudoku.set(row, col, value);
            self.collapse(cell, value);
            self.logic_process(&new_sudoku);
            if new_sudoku.is_solved() {
                return Some(new_sudoku.to_string());
            }
            if let Some(solution) = self.solve_recursive(&new_sudoku) {
                return Some(solution);
            }
            self.permutations = perm_state;
        }

        None
    }
}

impl Solver for WaveFunctionCollapseSolver {
    fn solve(&mut self, input: &Sudoku) -> Option<String> {
        self.generate_permutations(input);
        self.logic_process(input);
        self.solve_recursive(input)
    }
}
