use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Clone)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Clone)]
pub struct Scan {
    /// The ClamAV service host name (String)
    #[arg(short, long)]
    pub host: String,

    /// The ClamAV service port (Number)
    #[arg(short, long)]
    pub port: u8,

    /*
    /// The path of the calam config file
    #[arg(
        short,
        long,
        value_name = "CALAM_CONFIG_PATH",
        default_value = "./calam.yaml"
    )]
    pub config: Option<PathBuf>,
    */
    /// The file to scan path
    #[arg(short, long)]
    pub file: PathBuf,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Scan files
    ///
    /// This send files to remote ClamAV service
    Scan(Scan),
}
