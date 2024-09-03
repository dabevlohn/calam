mod cli;
mod clmd;

use eyre::Error;
use std::process::ExitCode;

use crate::cli::Cli;
use crate::clmd::clam_scan;
use clap::Parser;

fn run() -> eyre::Result<()> {
    let args = Cli::parse();

    match args.command {
        cli::Commands::Scan(scan) => {
            clam_scan(scan.host, scan.port, scan.file);
        } /*
          cli::Commands::Ping(ping) => {
              let config =
                  Config::read(&new.config).wrap_err("failed reading the gempost config file")?;

              create_new_post(&config.posts_dir, &new.slug, new.title.as_deref())
                  .wrap_err("failed creating new gemlog post")?;
          }
          */
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
