use tokio::net::TcpListener;

mod filereceiver;
use filereceiver::FileReceiver;

mod cli;
use crate::cli::Cli;
use clap::Parser;

async fn start() -> eyre::Result<()> {
    let args = Cli::parse();

    match args.command {
        cli::Commands::Fr(fr) => {
            let addr = format!("{}:{}", fr.address, fr.port).to_string();
            // let addr = "127.0.0.1:3310".to_string();
            let socket = TcpListener::bind(&addr).await.unwrap();
            FileReceiver::new(socket, fr.tempdir).run().await;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    start().await.unwrap();
}
