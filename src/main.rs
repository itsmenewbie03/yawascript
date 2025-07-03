use crate::utils::{cli::CommandType::*, parser::run};
use clap::Parser;
use utils::compiler::compile;

use crate::utils::cli::Cli;

mod utils;

fn main() {
    let args = Cli::parse();
    match args.command {
        Run(arg) => run(arg.file),
        Compile(arg) => compile(arg.file, arg.os, arg.arch),
    }
}
