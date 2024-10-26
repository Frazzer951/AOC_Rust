use clap::{Parser, Subcommand};

mod advent_of_code;
mod utils;
mod y2015;
mod y2022;
mod years;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files for year and day
    Add { year: u32, day: u32 },
    /// Run a specific year and day
    RunDay { year: u32, day: u32 },
    /// Run a specific year
    RunYear { year: u32 },
}

fn main() {
    let cli = Cli::parse();
    let aoc = advent_of_code::AdventOfCode::new();

    match &cli.command {
        Some(Commands::Add { year, day }) => {
            utils::create_year_day(*year, *day);
        }
        Some(Commands::RunDay { year, day }) => {
            aoc.run_year(
                *year,
                &years::Days {
                    all: false,
                    days: vec![*day],
                },
            );
        }
        Some(Commands::RunYear { year }) => {
            aoc.run_year(
                *year,
                &years::Days {
                    all: true,
                    days: vec![],
                },
            );
        }
        None => {
            aoc.run_all_years();
        }
    }
}
