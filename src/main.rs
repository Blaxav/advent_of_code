mod utils;
mod year_2018;
use clap::Parser;

#[derive(Parser)]
#[command(name = "Advent of code CLI in Rust")]
struct Cli {
    year: u64,
    day: u64,
    data: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.year {
        2018 => year_2018::run(cli.day, &cli.data),
        _ => println!("Year {} not implemented yet", cli.year),
    }
}
