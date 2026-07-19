use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(about, disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Evaluate and run wafl source code
    Run {
        /// An arbitrary wafl expression
        #[arg(short, long, exclusive = true)]
        expr: Option<String>,
        /// The path to an input file
        #[arg(short = 'F', long, exclusive = true)]
        file: Option<PathBuf>,
    },
    /// Launch an interactive REPL
    Repl,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Run { expr, file } => match (expr, file) {
            (Some(expr), None) => wafl_interp::eval(&expr),
            (None, Some(file)) => wafl_interp::eval_file(file),
            _ => unreachable!(),
        },
        Command::Repl => wafl_interp::repl(),
    }
}
