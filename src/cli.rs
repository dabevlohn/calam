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
    /// The ClamAV service or FileReceiver host name (String)
    #[arg(short, long, default_value = "localhost")]
    pub address: String,

    /// The ClamAV service or FileReceiver port (Number)
    #[arg(short, long, default_value = "3310")]
    pub port: u16,

    /// The file to scan path
    #[arg(short, long)]
    pub file: PathBuf,
}

#[derive(Args, Clone)]
pub struct FileReceiver {
    /// Receiver host name (String)
    #[arg(short, long, default_value = "localhost")]
    pub address: String,

    /// Receiver port (Number)
    #[arg(short, long, default_value = "3310")]
    pub port: u16,

    /// Path of temporary directory to save received files
    #[arg(short, long, default_value = "/tmp")]
    pub tempdir: PathBuf,

    /// QuickWit hostname
    #[arg(long, default_value = "localhost")]
    pub qwhost: String,

    /// QuickWit port
    #[arg(long, default_value = "7280")]
    pub qwport: u16,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Send filestreams to a clamd-analog service - FileReceiver (need to run it before)
    ///
    /// This send file streams to a remote ClamAV service or a FileReceiver
    Scan(Scan),
    /// Run a FileReceiver node that saves stream to file, put it to local filesystem or S3-storage and
    /// post it to several DLP systems via REST API. Scan results will be pushed into a QuickWit index.
    ///
    /// This accept file streams from remote ClamAV tool
    Fr(FileReceiver),
}
