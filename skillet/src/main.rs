use clap::Parser;
use std::env;
use std::path::PathBuf;

mod commands {
    pub mod conf;
}
mod config;

mod cli;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    if cli.markdown_help {
        println!("{}", clap_markdown::help_markdown::<Cli>());
        return;
    }

    // Change to requested directory, if any
    if let Some(dir) = &cli.cwd {
        std::env::set_current_dir(dir).expect("Failed to change directory");
    }

    let context = Context {
        quiet: cli.quiet,
        cwd: env::current_dir().unwrap(),
    };

    match cli.command {
        Some(Commands::Config(_args)) => {
            commands::conf::run(&context);
        }
        Some(Commands::Init(_args)) => {
            cmd_init(&context);
        }
        Some(Commands::Plan(_args)) => {
            cmd_plan(&context);
        }
        Some(Commands::Apply(_args)) => {
            cmd_apply(&context);
        }
        None => {
            eprintln!("No command provided. Use --help for usage.");
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
