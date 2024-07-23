use std::io::{stdout, Write};

mod grid;
mod solver;

const EASY_TEST: &str = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
const HARD_TEST: &str = "000000021090700000000000000000514000630000000000002000000600930001040000200000800";

fn main() {
    let mut stdout = stdout();

    let mut sudoku = grid::Grid::new();
    // sudoku.load_from_str(EASY_TEST, '0');
    // sudoku.load_from_str(HARD_TEST, '0');

    // println!("{}", sudoku);

    let mut solver = solver::BruteForceSolver::new();

    // let solution = sudoku.solve(&mut solver);

    // match solution {
    //     Some(grid) => println!("{}", grid),
    //     None => println!("No solution found"),
    // }

    let easy50 = std::fs::read_to_string("data/easy50.txt")
        .expect("Unable to open file")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    use std::time::Instant;
    let iter_count = 10;
    let start = Instant::now();

    for i in 0..iter_count {
        let sub_start = Instant::now();
        let mut j = 1;
        for puzzle in easy50.iter() {
            sudoku.load_from_str(puzzle, '0');
            sudoku.solve(&mut solver);

            print!("\r{:<4}/{:>4}", j, easy50.len());
            stdout.flush().unwrap();
            j += 1;
        }
        println!("\n{}: {:?}", i + 1, sub_start.elapsed());
    }

    let duration = start.elapsed();
    println!(
        "Time elapsed in solving 50 puzzles {} times: {:?} | Avg_50: {:?}",
        iter_count,
        duration,
        duration / iter_count
    );
}
