mod utils;
mod year_2018;
mod year_2024;

use aoc_client::get_data;
use clap::{Parser, Subcommand};
mod aoc_client;
use tokio;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Enum for the top-level commands
#[derive(Subcommand, Debug)]
enum Commands {
    Get {
        year: u64,
        day: u64,
        data_path: Option<String>,
    },
    Solve {
        year: u64,
        day: u64,
        data_path: Option<String>,
    },
    Submit {
        year: u64,
        day: u64,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Get {
            year,
            day,
            data_path,
        } => get_data(year, day, data_path).await,
        Commands::Solve {
            year,
            day,
            data_path,
        } => solve(year, day, data_path),
        Commands::Submit { year, day } => println!("Not implemented yet"),
    }
}

fn solve(year: u64, day: u64, data: Option<String>) {
    let data_path = match data {
        Some(data) => {
            if data == "test" {
                format!("./data/{}/test.txt", year)
            } else {
                data
            }
        }
        None => format!("./data/{}/{}.txt", year, day),
    };

    println!("Taking data at {}", data_path);

    match year {
        2018 => year_2018::run(day, &data_path),
        2024 => year_2024::run(day, &data_path),
        _ => println!("Year {} not implemented yet", year),
    };
}
