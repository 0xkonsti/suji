use suji;

const EASY_TEST: &str = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";

fn main() {
    let mut sudoku = suji::Sudoku::new(suji::BackendType::BitfieldGrid);

    sudoku.load_from_str(EASY_TEST, '0');

    println!("{}", sudoku);

    let mut solver = suji::BruteForceSolver::new();

    let solution = sudoku.solve(&mut solver);

    match solution {
        Some(grid) => {
            sudoku.load_from_str(&grid, '0');
            println!("{}", sudoku);
        }
        None => println!("No solution found"),
    }
}
