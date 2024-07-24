use clap::{Args, Parser, Subcommand};

/// YawaScript is not just a meme anymore.
#[derive(Debug, Parser)]
#[clap(author, version, about,long_about=None)]
pub struct Cli {
    #[clap(subcommand)]
    command: CommandType,
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
    Run(RunCommand),
}

#[derive(Debug, Args)]
pub struct RunCommand {
    /// The file to run
    file: std::path::PathBuf,
}
