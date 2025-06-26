use clap::{Args, Parser, Subcommand};
use std::env;
use std::path::PathBuf;
mod commands {
    pub mod conf;
}
mod config;

/// tfaid: Terraform wrapper CLI
#[derive(Parser, Debug)]
#[command(name = "tfaid")]
#[command(about = "Terraform wrapper CLI", version)]
struct Cli {
    /// Suppress global process output; only errors will be shown.
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Change to this directory before running any commands.
    #[arg(short = 'C', long = "directory")]
    cwd: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Config(ConfigArgs),
    Init(InitArgs),
    Plan(PlanArgs),
    Apply(ApplyArgs),
}

#[derive(Args, Debug)]
struct ConfigArgs {}

#[derive(Args, Debug)]
struct InitArgs {}

#[derive(Args, Debug)]
struct PlanArgs {}

#[derive(Args, Debug)]
struct ApplyArgs {}

fn main() {
    let cli = Cli::parse();

    // Change to requested directory, if any
    if let Some(dir) = &cli.cwd {
        std::env::set_current_dir(dir).expect("Failed to change directory");
    }

    // Simulate ctx.obj equivalent
    let context = Context {
        quiet: cli.quiet,
        cwd: env::current_dir().unwrap(),
    };

    match cli.command {
        Commands::Config(_args) => {
            commands::conf::run(&context);
        }
        Commands::Init(_args) => {
            cmd_init(&context);
        }
        Commands::Plan(_args) => {
            cmd_plan(&context);
        }
        Commands::Apply(_args) => {
            cmd_apply(&context);
        }
    }
}

#[derive(Debug)]
struct Context {
    quiet: bool,
    cwd: PathBuf,
}

// Example stub functions
fn cmd_init(ctx: &Context) {
    if !ctx.quiet {
        println!("Running 'init' in {:?}", ctx.cwd);
    }
    // future: bootstrap(ctx) logic here
}

fn cmd_plan(ctx: &Context) {
    if !ctx.quiet {
        println!("Running 'plan' in {:?}", ctx.cwd);
    }
}

fn cmd_apply(ctx: &Context) {
    if !ctx.quiet {
        println!("Running 'apply' in {:?}", ctx.cwd);
    }
}
