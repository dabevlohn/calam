use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

mod filereceiver;
use filereceiver::FileReceiver;

mod streamsender;
use streamsender::StreamSender;

mod trackeractor;
use trackeractor::{TrackerActor, TrackerMessage};

mod indexingestor;

mod cli;
use crate::cli::Cli;

use clap::Parser;

//const PING: &[u8; 6] = b"zPING\0";
//const VERSION: &[u8; 9] = b"zVERSION\0";
const INSTREAM: &[u8; 10] = b"zINSTREAM\0";
const END_OF_STREAM: &[u8; 4] = &[0, 0, 0, 0];

async fn start() {
    let args = Cli::parse();

    match args.command {
        cli::Commands::Fr(fr) => {
            let addr = format!("{}:{}", fr.address, fr.port).to_string();
            let socket = TcpListener::bind(&addr).await.unwrap();
            let (tracker_tx, tracker_rx) = mpsc::channel::<TrackerMessage>(1);
            tokio::spawn(async move {
                TrackerActor::new(tracker_rx, fr.qwhost, fr.qwport)
                    .run()
                    .await;
            });
            FileReceiver::new(socket, fr.tempdir)
                .run(tracker_tx.clone())
                .await;
        }
        cli::Commands::Scan(scan) => {
            let addr = format!("{}:{}", scan.address, scan.port).to_string();
            let stream = TcpStream::connect(&addr).await.unwrap();
            StreamSender::new(stream, scan.file).clam_scan().await;
        }
    }
}

#[tokio::main]
async fn main() {
    start().await;
}
