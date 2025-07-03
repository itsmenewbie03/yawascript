use clap::{Args, Parser, Subcommand};

/// YawaScript is not just a meme anymore.
#[derive(Debug, Parser)]
#[clap(author, version, about,long_about=None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: CommandType,
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
    /// Run a file
    Run(RunCommand),
    Compile(CompileCommand),
}

#[derive(Debug, Args)]
pub struct RunCommand {
    /// The file to run
    pub file: std::path::PathBuf,
}

#[derive(Debug, Args)]
pub struct CompileCommand {
    /// The file to compile
    pub file: std::path::PathBuf,
    /// Target operating system, e.g. linux, windows, darwin
    #[arg(long)]
    pub os: Option<String>,

    /// Target architecture, e.g. x86_64, arm, aarch64
    #[arg(long)]
    pub arch: Option<String>,
}
