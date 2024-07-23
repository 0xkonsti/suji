mod brute_force_solver;

use crate::grid::Grid;
pub use brute_force_solver::BruteForceSolver;

pub trait Solver {
    fn solve(&mut self, input: &Grid) -> Option<Grid>;
}
