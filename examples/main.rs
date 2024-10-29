use std::time::Instant;
use suji;

const EASY_TEST: &str = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
const HARD_TEST: &str = "900050200400000780000087600000360000005902400000018000004820000051000002006090007";
const EVIL_TEST: &str = "000000000000003085001020000000507000004000100090000000500000073002010000000040009";

fn main() {
    let mut sudoku = suji::Sudoku::new(suji::BackendType::BitfieldGrid);

    sudoku.load_from_str(EASY_TEST, '0');
    sudoku.load_from_str(HARD_TEST, '0');
    sudoku.load_from_str(EVIL_TEST, '0');

    println!("{}", sudoku);

    //let mut solver = suji::BruteForceSolver::new();
    let mut solver = suji::WaveFunctionCollapseSolver::new();

    let now = Instant::now();
    let solution = sudoku.solve(&mut solver);
    println!(
        "Elapsed time {{\n    seconds: {},\n    millis:  {},\n    micros:  {},\n    nanos:   {}\n}}",
        now.elapsed().as_secs(),
        now.elapsed().as_millis(),
        now.elapsed().as_micros(),
        now.elapsed().as_nanos()
    );

    match solution {
        Some(grid) => {
            sudoku.load_from_str(&grid, '0');
            println!("{}", sudoku);
        }
        None => println!("No solution found"),
    }
}
