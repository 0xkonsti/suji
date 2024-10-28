fn main() {
    let data = std::fs::read_to_string("data/easy50.txt").unwrap();
    let puzzles = data.lines().collect::<Vec<_>>();

    let mut sudoku = suji::Sudoku::new(suji::BackendType::BitfieldGrid);
    let mut solver = suji::WaveFunctionCollapseSolver::new();
    //let mut solver = suji::BruteForceSolver::new();

    let now = std::time::Instant::now();
    for puzzle in puzzles {
        sudoku.load_from_str(puzzle, '0');
        sudoku.solve(&mut solver);
    }
    println!(
        "Elapsed time {{\n    seconds: {},\n    millis:  {},\n    micros:  {},\n    nanos:   {}\n}}",
        now.elapsed().as_secs(),
        now.elapsed().as_millis(),
        now.elapsed().as_micros(),
        now.elapsed().as_nanos()
    );
}
