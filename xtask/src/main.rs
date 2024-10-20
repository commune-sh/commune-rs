use std::process::ExitCode;

use clap::Parser;
use miette::{Context, IntoDiagnostic};

#[derive(clap::Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {}

fn main() -> ExitCode {
    let main = || -> miette::Result<()> {
        let _args = Args::parse();

        let _shell = xshell::Shell::new().into_diagnostic().wrap_err("failed")?;

        Ok(())
    };

    match main() {
        Err(error) => {
            eprintln!("\n{error:?}");

            ExitCode::FAILURE
        }
        Ok(_) => ExitCode::SUCCESS,
    }
}
