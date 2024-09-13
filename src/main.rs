use std::time::Duration;
use tokio::net::TcpListener;
use tokio::{task, time};

mod filereceiver;
use filereceiver::FileReceiver;

const END_OF_STREAM: &[u8; 4] = &[0, 0, 0, 0];

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let addr = "127.0.0.1:3310".to_string();
    let socket = TcpListener::bind(&addr).await.unwrap();
    //task::spawn(async move {
    FileReceiver::new(socket).run().await;
    //time::sleep(Duration::from_secs(3)).await;
    //});
    //loop {
    //    time::sleep(Duration::from_secs(120)).await;
    //}
}
