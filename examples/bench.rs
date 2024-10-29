fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let null_char = std::env::args().nth(2).unwrap().chars().next().unwrap();

    let data = std::fs::read_to_string(file_path).unwrap();
    let mut puzzles = data.lines();
    let length = puzzles.next().unwrap().parse::<usize>().unwrap();

    let mut sudoku = suji::Sudoku::new(suji::BackendType::BitfieldGrid);
    let mut solver = suji::WaveFunctionCollapseSolver::new();

    let now = std::time::Instant::now();
    let mut total_guesses = 0;
    for puzzle in puzzles {
        sudoku.load_from_str(puzzle, null_char);
        sudoku.solve(&mut solver);
        total_guesses += solver.get_guesses();
    }
    println!(
        "Elapsed time {{\n    seconds: {},\n    millis:  {},\n    micros:  {},\n    nanos:   {}\n}}",
        now.elapsed().as_secs(),
        now.elapsed().as_millis(),
        now.elapsed().as_micros(),
        now.elapsed().as_nanos()
    );

    println!("| puzzels/s | us/puzzle | guesses/puzzle |");
    println!(
        "| {:<9.1} | {:<9.1} | {:<14.1} |",
        length as f64 / now.elapsed().as_secs_f64(),
        now.elapsed().as_micros() as f64 / length as f64,
        total_guesses as f64 / length as f64
    );
}
