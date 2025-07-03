use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "skillet", about = "Terraform wrapper CLI", version)]
pub struct Cli {
    /// Suppress global process output; only errors will be shown.
    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,

    /// Change to this directory before running any commands.
    #[arg(short = 'C', long = "directory")]
    pub cwd: Option<PathBuf>,

    /// Output the CLI help in Markdown format
    #[arg(long, hide = true)]
    pub markdown_help: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Config(ConfigArgs),
    Init(InitArgs),
    Plan(PlanArgs),
    Apply(ApplyArgs),
}

#[derive(Args, Debug)]
pub struct ConfigArgs {}
#[derive(Args, Debug)]
pub struct InitArgs {}
#[derive(Args, Debug)]
pub struct PlanArgs {}
#[derive(Args, Debug)]
pub struct ApplyArgs {}
