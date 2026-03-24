use std::env;
use sukaku_rs::{Grid, Rater, Solver};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <puzzle>", args[0]);
        eprintln!("  puzzle: 81 characters (0-9 or . for empty)");
        std::process::exit(1);
    }

    let puzzle_str = &args[1];

    let grid = match Grid::parse(puzzle_str) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut solver = Solver::new(grid);
    let mut rater = Rater::new(&mut solver);
    let rating = rater.analyse();

    println!("ER: {:.1} ({})", rating.er, rating.er_technique);
}
