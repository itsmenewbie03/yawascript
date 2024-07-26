use crate::utils::{cli::CommandType::Run, parser::run};
use clap::Parser;

use crate::utils::cli::Cli;

mod utils;

fn main() {
    let args = Cli::parse();
    match args.command {
        Run(arg) => run(arg.file),
    }
}
