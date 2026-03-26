use clap::{Parser, Subcommand};
use sukaku_rs::{DifficultyRating, Generator, Grid, Rater, Solver, Symmetry};

#[derive(Parser)]
#[command(name = "sukaku-rs")]
#[command(about = "Sudoku puzzle generator and rater", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(help = "Puzzle string (81 characters)")]
    puzzle: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate new puzzles
    Generate {
        #[arg(long, default_value = "0.0")]
        min_diff: f64,

        #[arg(long, default_value = "10.0")]
        max_diff: f64,

        #[arg(long, default_value = "1")]
        count: usize,

        #[arg(long, default_value = "none")]
        symmetry: String,

        #[arg(long)]
        output: Option<String>,
    },

    /// Rate puzzles from file or string
    Rate {
        #[arg(long)]
        input: Option<String>,

        #[arg(long, help = "Puzzle string (81 characters)")]
        puzzle: Option<String>,

        #[arg(long)]
        output: Option<String>,

        #[arg(long, default_value = "%r,%p,%d,%g")]
        format: String,

        #[arg(long)]
        json: bool,
    },
}

fn parse_symmetry(s: &str) -> Symmetry {
    match s.to_lowercase().as_str() {
        "none" => Symmetry::None,
        "rotational-180" | "r180" => Symmetry::Rotational180,
        "rotational-90" | "r90" => Symmetry::Rotational90,
        "horizontal" | "h" => Symmetry::Horizontal,
        "vertical" | "v" => Symmetry::Vertical,
        "diagonal-main" | "dm" => Symmetry::DiagonalMain,
        "diagonal-anti" | "da" => Symmetry::DiagonalAnti,
        "full" => Symmetry::Full,
        _ => {
            eprintln!("Unknown symmetry: {}, using none", s);
            Symmetry::None
        }
    }
}

fn format_output(format: &str, rating: &DifficultyRating, puzzle: &str) -> String {
    format
        .replace("%r", &format!("{:.1}", rating.er))
        .replace("%p", &format!("{:.1}", rating.ep))
        .replace("%d", &format!("{:.1}", rating.ed))
        .replace("%t", &rating.er_technique)
        .replace("%g", puzzle)
}

fn cmd_generate(
    min_diff: f64,
    max_diff: f64,
    count: usize,
    symmetry: String,
    output: Option<String>,
) {
    let sym = parse_symmetry(&symmetry);
    let mut gen = Generator::with_difficulty(min_diff, max_diff);
    gen.require_unique = true;
    if sym != Symmetry::None {
        gen.symmetry = sym;
    }

    let mut puzzles = Vec::new();
    for i in 0..count {
        match gen.generate() {
            Ok(grid) => {
                puzzles.push(grid.to_string().replace('\n', ""));
            }
            Err(e) => {
                eprintln!("Generation {} failed: {}", i + 1, e);
            }
        }
    }

    let output_str = puzzles.join("\n");

    if let Some(path) = output {
        std::fs::write(&path, &output_str).unwrap();
        println!("Wrote {} puzzles to {}", puzzles.len(), path);
    } else {
        println!("{}", output_str);
    }
}

fn cmd_rate(
    input: Option<String>,
    puzzle_arg: Option<String>,
    output: Option<String>,
    format: String,
    json: bool,
) {
    let puzzles: Vec<String> = if let Some(path) = input {
        std::fs::read_to_string(&path)
            .unwrap()
            .lines()
            .filter(|l| l.len() >= 81)
            .map(|s| s.to_string())
            .collect()
    } else if let Some(puzzle) = puzzle_arg {
        vec![puzzle]
    } else {
        vec![]
    };

    if puzzles.is_empty() && !json {
        eprintln!("No puzzles to rate. Use --input <file> or provide puzzle as argument.");
        std::process::exit(1);
    }

    let output_str = if json {
        let mut json_results = Vec::new();
        for puzzle in &puzzles {
            let grid = match Grid::parse(puzzle) {
                Ok(g) => g,
                Err(e) => {
                    eprintln!("Invalid puzzle {}: {}", puzzle, e);
                    continue;
                }
            };

            let mut solver = Solver::new(grid);
            let mut rater = Rater::new(&mut solver);
            let rating = rater.analyse();

            json_results.push(serde_json::json!({
                "puzzle": puzzle,
                "er": rating.er,
                "ep": rating.ep,
                "ed": rating.ed,
                "technique": rating.er_technique,
            }));
        }
        serde_json::to_string_pretty(&json_results).unwrap()
    } else {
        let mut text_results = Vec::new();
        for puzzle in &puzzles {
            let grid = match Grid::parse(puzzle) {
                Ok(g) => g,
                Err(e) => {
                    eprintln!("Invalid puzzle {}: {}", puzzle, e);
                    continue;
                }
            };

            let mut solver = Solver::new(grid);
            let mut rater = Rater::new(&mut solver);
            let rating = rater.analyse();

            let formatted = format_output(&format, &rating, puzzle);
            text_results.push(formatted);
        }
        text_results.join("\n")
    };

    if let Some(path) = output {
        std::fs::write(&path, &output_str).unwrap();
        if json {
            println!("Wrote results to {}", path);
        } else {
            let count = output_str.lines().count();
            println!("Wrote {} results to {}", count, path);
        }
    } else {
        println!("{}", output_str);
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Generate {
            min_diff,
            max_diff,
            count,
            symmetry,
            output,
        }) => {
            cmd_generate(min_diff, max_diff, count, symmetry, output);
        }
        Some(Commands::Rate {
            input,
            puzzle,
            output,
            format,
            json,
        }) => {
            cmd_rate(input, puzzle, output, format, json);
        }
        None => {
            if let Some(puzzle_str) = cli.puzzle {
                let grid = match Grid::parse(&puzzle_str) {
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
                println!("EP: {:.1}", rating.ep);
                println!("ED: {:.1}", rating.ed);
            } else {
                eprintln!("Usage: sukaku-rs <puzzle>");
                eprintln!("       sukaku-rs generate [OPTIONS]");
                eprintln!("       sukaku-rs rate --input <file> [OPTIONS]");
                std::process::exit(1);
            }
        }
    }
}
