use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Clone)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Clone)]
pub struct FileReceiver {
    /// Receiver host name (String)
    #[arg(short, long, default_value = "localhost")]
    pub address: String,

    /// Receiver port (Number)
    #[arg(short, long, default_value = "3310")]
    pub port: u16,

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
    /// This send files to remote ClamAV service
    Fr(FileReceiver),
}
