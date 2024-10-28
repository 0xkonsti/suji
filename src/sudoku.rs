use crate::backend::{BackendType, CloneableBackend};
use crate::solver::Solver;

#[derive(Debug)]
pub struct Sudoku {
    backend: Box<dyn CloneableBackend>,
}

impl Sudoku {
    pub fn new(backend: BackendType) -> Self {
        match backend {
            BackendType::BitfieldGrid => Self {
                backend: Box::new(crate::backend::BitfieldGrid::new()),
            },
        }
    }

    pub fn new_custom(backend: Box<dyn CloneableBackend>) -> Self {
        Self {
            backend,
        }
    }

    pub fn load_from_str(&mut self, input: &str, null_chr: char) {
        self.backend.load_from_str(input, null_chr);
    }

    pub fn to_string(&self) -> String {
        self.backend.to_string('0')
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.backend.get(row, col)
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        self.backend.set(row, col, value)
    }

    pub fn unset(&mut self, row: usize, col: usize) {
        self.backend.unset(row, col)
    }

    pub fn get_empty_cells(&self) -> &Vec<(usize, usize)> {
        self.backend.get_empty_cells()
    }

    pub fn is_valid(&self) -> bool {
        self.backend.is_valid()
    }

    pub fn is_solved(&self) -> bool {
        self.backend.get_empty_cells().is_empty() && self.is_valid()
    }

    pub fn solve(&mut self, solver: &mut dyn Solver) -> Option<String> {
        solver.solve(self)
    }

    pub fn is_possible_value(&self, row: usize, col: usize, value: u8) -> bool {
        self.backend.is_possible_value(row, col, value)
    }

    pub fn get_possible_values(&self, row: usize, col: usize) -> u16 {
        self.backend.get_possible_values(row, col)
    }
}

impl Clone for Sudoku {
    fn clone(&self) -> Self {
        Self {
            backend: self.backend.clone_box(),
        }
    }
}

const TOP_ROW: &str = "╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗";
const MIDDLE_ROW_DOUBLE: &str = "╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣";
const MIDDLE_ROW_SINGLE: &str = "╟───┼───┼───╫───┼───┼───╫───┼───┼───╢";
const BOTTOM_ROW: &str = "╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝";
const STRAIGHT_DOUBLE: &str = "║";
const STRAIGHT_SINGLE: &str = "│";
const EMPTY: &str = "   ";

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(TOP_ROW);
        result.push('\n');

        for i in 0..9 {
            result.push_str("║");
            for j in 0..9 {
                let value = self.get(i, j);
                if value == 0 {
                    result.push_str(EMPTY);
                } else {
                    result.push_str(&format!(" {} ", value));
                }

                if j == 2 || j == 5 {
                    result.push_str(STRAIGHT_DOUBLE);
                } else if j != 8 {
                    result.push_str(STRAIGHT_SINGLE);
                }
            }
            result.push_str(STRAIGHT_DOUBLE);
            result.push('\n');

            if i == 2 || i == 5 {
                result.push_str(MIDDLE_ROW_DOUBLE);
                result.push('\n');
            } else if i != 8 {
                result.push_str(MIDDLE_ROW_SINGLE);
                result.push('\n');
            }
        }

        result.push_str(BOTTOM_ROW);

        write!(f, "{}", result)
    }
}
