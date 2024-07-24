use clap::Parser;

use crate::utils::cli::Cli;

mod utils;

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}
