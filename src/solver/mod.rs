use crate::Sudoku;

mod brute_force_solver;

pub use brute_force_solver::BruteForceSolver;

pub trait Solver {
    fn solve(&mut self, input: &Sudoku) -> Option<String>;
}
