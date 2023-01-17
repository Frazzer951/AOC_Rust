use clap::{Parser, Subcommand};

pub mod utils;
pub mod y2022;
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

    if let Some(command) = &cli.command {
        match command {
            Commands::Add { year, day } => {
                utils::create_year_day(*year, *day);
            },
            Commands::RunDay { year, day } => {
                years::run_years(years::Years::new().add_year(years::Year::new(*year).set_day(*day)))
            },
            Commands::RunYear { year } => years::run_years(years::Years::new().add_year(years::Year::new(*year))),
        }
    } else {
        years::run_years(years::Years::new().add_year(years::Year::new(2022)))
    }
}
