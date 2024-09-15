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
    #[arg(short, long, default_value = "localhost")]
    pub address: String,

    /// The ClamAV service port (Number)
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

    /// MQTT Broker hostname
    #[arg(long, default_value = "localhost")]
    pub brokerhost: String,

    /// MQTT Broker port
    #[arg(long, default_value = "1883")]
    pub brokerport: u16,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Scan files
    ///
    /// This send file streams to remote ClamAV service
    Scan(Scan),
    /// File Receiver
    ///
    /// This accept file streams from remote ClamAV tool
    Fr(FileReceiver),
}
