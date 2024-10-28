use crate::Sudoku;

mod brute_force_solver;
mod wave_function_collapse_solver;

pub use brute_force_solver::BruteForceSolver;
pub use wave_function_collapse_solver::WaveFunctionCollapseSolver;

pub trait Solver {
    fn solve(&mut self, input: &Sudoku) -> Option<String>;
}
