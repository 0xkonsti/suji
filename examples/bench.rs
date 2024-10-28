fn main() {
    let data = std::fs::read_to_string("data/bench.txt").unwrap();
    let mut puzzles = data.lines();
    let length = puzzles.next().unwrap().parse::<usize>().unwrap();

    let mut sudoku = suji::Sudoku::new(suji::BackendType::BitfieldGrid);
    let mut solver = suji::WaveFunctionCollapseSolver::new();

    let now = std::time::Instant::now();
    for (i, puzzle) in puzzles.enumerate() {
        sudoku.load_from_str(puzzle, '0');
        sudoku.solve(&mut solver);
        println!("{:<6} / {:<6}", i, length);
    }
    println!(
        "Elapsed time {{\n    seconds: {},\n    millis:  {},\n    micros:  {},\n    nanos:   {}\n}}",
        now.elapsed().as_secs(),
        now.elapsed().as_millis(),
        now.elapsed().as_micros(),
        now.elapsed().as_nanos()
    );
}
