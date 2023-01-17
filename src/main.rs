use clap::{Parser, Subcommand};

pub mod utils;
mod y2022;

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
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = &cli.command {
        match command {
            Commands::Add { year, day } => {
                utils::create_year_day(*year, *day);
            },
        }
    } else {
        run_years(Years { y2022: true })
    }
}

struct Years {
    y2022: bool,
}

fn run_years(years: Years) {
    if years.y2022 {
        y2022()
    }
}

fn y2022() {
    println!("2022");
    y2022::day_1::run();
    y2022::day_2::run();
    y2022::day_3::run();
    y2022::day_4::run();
    y2022::day_5::run();
}
