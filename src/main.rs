mod actor;
mod cli;
mod clmd;
mod message;

use eyre::Error;
use std::process::ExitCode;

use crate::cli::Cli;
use crate::clmd::clam_scan;
use clap::Parser;

fn run() -> eyre::Result<()> {
    let args = Cli::parse();

    match args.command {
        cli::Commands::Scan(scan) => {
            clam_scan(scan.address, scan.port, scan.file).expect("cannot scan file");
        }
    }

    Ok(())
}

fn main() -> eyre::Result<ExitCode> {
    color_eyre::install()?;

    if let Err(err) = run() {
        // User-facing errors should not show a stack trace.
        if let Some(user_err) = err.downcast_ref::<Error>() {
            eprintln!("{}", user_err);
            return Ok(ExitCode::FAILURE);
        }

        return Err(err);
    }

    Ok(ExitCode::SUCCESS)
}
