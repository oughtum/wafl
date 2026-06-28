use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};
use wafl_interp::interpreter::Interpreter;

#[derive(clap::Parser)]
#[command(about, disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Evaluate and run WAFL source code
    Run {
        /// An arbitrary WAFL expression
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
        Command::Run { expr, file } => {
            let (src, path, name, expr) = if let Some(expr) = expr {
                (expr, None, Some("expr"), true)
            } else if let Some(path) = file {
                (fs::read_to_string(&path).unwrap(), Some(path), None, false)
            } else {
                unreachable!("")
            };

            Interpreter.run(src, name, path, expr)
        }
        Command::Repl => Interpreter.repl(),
    }
}
