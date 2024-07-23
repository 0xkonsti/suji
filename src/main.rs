mod grid;
mod solver;

const EASY_TEST: &str = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
const HARD_TEST: &str = "000000021090700000000000000000514000630000000000002000000600930001040000200000800";

fn main() {
    let mut sudoku = grid::Grid::new();
    sudoku.load_from_str(EASY_TEST, '0');
    // sudoku.load_from_str(HARD_TEST, '0');

    println!("{}", sudoku);

    let mut solver = solver::BruteForceSolver::new();

    let solution = sudoku.solve(&mut solver);

    match solution {
        Some(grid) => println!("{}", grid),
        None => println!("No solution found"),
    }
}
