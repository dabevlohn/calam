use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

mod modules;

use clap::Parser;
use modules::cli::{Cli, Commands};
use modules::filereceiver::FileReceiver;
use modules::streamsender::StreamSender;
use modules::trackeractor::{TrackerActor, TrackerMessage};

async fn start() {
    let args = Cli::parse();

    match args.command {
        Commands::Fr(fr) => {
            let addr = format!("{}:{}", fr.address, fr.port).to_string();
            let socket = TcpListener::bind(&addr).await.unwrap();
            let (tracker_tx, tracker_rx) = mpsc::channel::<TrackerMessage>(1);
            tokio::spawn(async move {
                TrackerActor::new(tracker_rx).run().await;
            });
            FileReceiver::new(socket, fr.tempdir)
                .run(tracker_tx.clone())
                .await;
        }
        Commands::Scan(scan) => {
            let addr = format!("{}:{}", scan.address, scan.port).to_string();
            let stream = TcpStream::connect(&addr).await.unwrap();
            StreamSender::new(stream, scan.file).clam_scan().await;
        }
    }
}

#[tokio::main]
async fn main() {
    // TODO: implement logging
    //
    start().await;
}
