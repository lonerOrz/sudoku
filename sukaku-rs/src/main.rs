use clap::{Parser, Subcommand};
use rayon::prelude::*;
use std::time::Instant;
use sukaku_rs::{DifficultyRating, Generator, Grid, Rater, Solver, Symmetry};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "sukaku-rs")]
#[command(about = "Sudoku puzzle generator and rater", long_about = None)]
#[command(version = VERSION)]
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

        #[arg(long, help = "Number of threads for parallel generation")]
        threads: Option<usize>,
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

        #[arg(long, help = "Terminate if not pearl (EP) rating")]
        pearl: bool,

        #[arg(long, help = "Terminate if not diamond (ED) rating")]
        diamond: bool,

        #[arg(long, help = "Show total processing time")]
        total_time: bool,

        #[arg(long, help = "Format string to print before processing")]
        start: Option<String>,

        #[arg(long, help = "Format string to print before each puzzle")]
        before: Option<String>,

        #[arg(long, help = "Format string to print after each puzzle")]
        after: Option<String>,
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
    let mut result = format
        .replace("%r", &format!("{:.1}", rating.er))
        .replace("%p", &format!("{:.1}", rating.ep))
        .replace("%d", &format!("{:.1}", rating.ed))
        .replace("%D", &rating.er_technique) // Diamond technique name
        .replace("%P", &rating.er_technique) // Pearl technique name
        .replace("%R", &rating.er_technique) // Rating technique name
        .replace("%i", puzzle) // Puzzle grid
        .replace("%g", puzzle) // Puzzle string
        .replace("%l", "\n") // Newline
        .replace("%%", "%"); // Literal %

    // Technique name (%t) must be replaced before Tab (%T)
    result = result.replace("%t", &rating.er_technique);
    result = result.replace("%T", "\t"); // Tab
    result
}

fn format_output_with_time(
    format: &str,
    rating: &DifficultyRating,
    puzzle: &str,
    elapsed: f64,
    ordinal: usize,
) -> String {
    let mut result = format
        .replace("%r", &format!("{:.1}", rating.er))
        .replace("%p", &format!("{:.1}", rating.ep))
        .replace("%d", &format!("{:.1}", rating.ed))
        .replace("%D", &rating.er_technique)
        .replace("%P", &rating.er_technique)
        .replace("%R", &rating.er_technique)
        .replace("%i", puzzle)
        .replace("%g", puzzle)
        .replace("%l", "\n")
        .replace("%%", "%")
        .replace("%e", &format!("{:.3}", elapsed))
        .replace("%n", &ordinal.to_string());

    result = result.replace("%t", &rating.er_technique);
    result = result.replace("%T", "\t");
    result
}

fn cmd_generate(
    min_diff: f64,
    max_diff: f64,
    count: usize,
    symmetry: String,
    output: Option<String>,
    threads: Option<usize>,
) {
    let sym = parse_symmetry(&symmetry);

    // Set number of threads for rayon
    if let Some(num_threads) = threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .unwrap();
    }

    // Generate puzzles in parallel
    let puzzles: Vec<String> = (0..count)
        .into_par_iter()
        .map(|i| {
            // Use different seed for each iteration
            let mut gen = Generator::with_seed(i as u64);
            gen.require_unique = true;
            gen.min_difficulty = min_diff;
            gen.max_difficulty = max_diff;
            if sym != Symmetry::None {
                gen.symmetry = sym;
            }

            match gen.generate() {
                Ok(grid) => grid.to_string().replace('\n', ""),
                Err(e) => {
                    eprintln!("Generation {} failed: {}", i + 1, e);
                    String::new()
                }
            }
        })
        .filter(|s| !s.is_empty())
        .collect();

    let output_str = puzzles.join("\n");

    if let Some(path) = output {
        std::fs::write(&path, &output_str).unwrap();
        println!("Wrote {} puzzles to {}", puzzles.len(), path);
    } else {
        println!("{}", output_str);
    }
}

struct RateOptions {
    input: Option<String>,
    puzzle: Option<String>,
    output: Option<String>,
    format: String,
    json: bool,
    pearl: bool,
    diamond: bool,
    show_total_time: bool,
    start: Option<String>,
    before: Option<String>,
    after: Option<String>,
}

fn cmd_rate_opts(opts: RateOptions) {
    let RateOptions {
        input,
        puzzle: puzzle_arg,
        output,
        format,
        json,
        pearl,
        diamond,
        show_total_time,
        start,
        before,
        after,
    } = opts;

    let start_time = Instant::now();

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

    let mut skipped = 0;
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

            // Check pearl/diamond filters
            // Pearl: EP >= 6.0, Diamond: ED >= 7.0 (SukakuExplainer thresholds)
            if pearl && rating.ep < 6.0 {
                skipped += 1;
                continue;
            }
            if diamond && rating.ed < 7.0 {
                skipped += 1;
                continue;
            }

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

            // Check pearl/diamond filters (non-JSON branch)
            // Pearl: EP >= 6.0, Diamond: ED >= 7.0 (SukakuExplainer thresholds)
            if pearl && rating.ep < 6.0 {
                skipped += 1;
                continue;
            }
            if diamond && rating.ed < 7.0 {
                skipped += 1;
                continue;
            }

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
        if !output_str.is_empty() {
            println!("{}", output_str);
        }
    }

    if show_total_time {
        let elapsed = start_time.elapsed();
        eprintln!("Total time: {:.2}s", elapsed.as_secs_f64());
    }

    if skipped > 0 {
        eprintln!("Skipped {} puzzles (filtered by pearl/diamond)", skipped);
    }

    if output_str.is_empty() && skipped > 0 {
        std::process::exit(0);
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
            threads,
        }) => {
            cmd_generate(min_diff, max_diff, count, symmetry, output, threads);
        }
        Some(Commands::Rate {
            input,
            puzzle,
            output,
            format,
            json,
            pearl,
            diamond,
            total_time,
            start,
            before,
            after,
        }) => {
            cmd_rate_opts(RateOptions {
                input,
                puzzle,
                output,
                format,
                json,
                pearl,
                diamond,
                show_total_time: total_time,
                start,
                before,
                after,
            });
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
                eprintln!("       sukaku-rs --version");
                std::process::exit(1);
            }
        }
    }
}
