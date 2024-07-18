mod grid;

const EASY_TEST: &str = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";

fn main() {
    let mut sudoku = grid::Grid::new();
    sudoku.load_from_str(EASY_TEST, '0');

    println!("{}", sudoku);
}
