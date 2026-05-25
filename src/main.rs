use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use ochre_interp::{diagnostics::Result, interpreter::Interpreter};

#[derive(clap::Parser)]
#[command(about, disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Evaluate and run Ochre source code
    Run {
        /// An Ochre expression
        #[arg(short, long, exclusive = true)]
        expr: Option<String>,
        /// The path to an Ochre module
        #[arg(short, long, exclusive = true)]
        module: Option<PathBuf>,
    },
    /// Launch an interactive REPL
    Repl,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Run { expr, module } => {
            let (src, path, name) = if let Some(expr) = expr {
                (expr, None, Some("expr"))
            } else if let Some(module) = module {
                (fs::read_to_string(&module).unwrap(), Some(module), None)
            } else {
                panic!("")
            };

            Interpreter::default().run(src, path, name)
        }
        Command::Repl => todo!(),
    }
}
