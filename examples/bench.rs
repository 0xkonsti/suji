use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let null_char = std::env::args().nth(2).unwrap().chars().next().unwrap();

    let data = std::fs::read_to_string(file_path).unwrap();
    let mut puzzles = data.lines();
    let length = puzzles.next().unwrap().parse::<usize>().unwrap();

    let now = std::time::Instant::now();
    let total_guesses = AtomicU64::new(0);
    puzzles.par_bridge().for_each(|puzzle| {
        let mut sudoku = suji::Sudoku::new(suji::BackendType::BitfieldGrid);
        let mut solver = suji::WaveFunctionCollapseSolver::new();
        sudoku.load_from_str(puzzle, null_char);
        sudoku.solve(&mut solver);
        total_guesses.fetch_add(solver.get_guesses(), Ordering::Relaxed);
    });
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
        total_guesses.load(Ordering::Relaxed) as f64 / length as f64
    );
}
