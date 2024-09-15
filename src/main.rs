use tokio::net::TcpListener;
use tokio::net::TcpStream;

mod filereceiver;
use filereceiver::FileReceiver;

mod streamsender;
use streamsender::StreamSender;

mod cli;
use crate::cli::Cli;

use clap::Parser;

//const PING: &[u8; 6] = b"zPING\0";
//const VERSION: &[u8; 9] = b"zVERSION\0";
const INSTREAM: &[u8; 10] = b"zINSTREAM\0";
const END_OF_STREAM: &[u8; 4] = &[0, 0, 0, 0];

async fn start() -> eyre::Result<()> {
    let args = Cli::parse();

    match args.command {
        cli::Commands::Fr(fr) => {
            let addr = format!("{}:{}", fr.address, fr.port).to_string();
            let socket = TcpListener::bind(&addr).await.unwrap();
            FileReceiver::new(socket, fr.tempdir).run().await;
        }
        cli::Commands::Scan(scan) => {
            let addr = format!("{}:{}", scan.address, scan.port).to_string();
            let stream = TcpStream::connect(&addr).await.unwrap();
            StreamSender::new(stream, scan.file).clam_scan().await;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    start().await.unwrap();
}
