use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::mpsc;

mod message;
use message::{FileOrder, Message, OrderBookActor};
mod tracker;
use tracker::{TrackerActor, TrackerMessage, VersionTrackerActor};

const END_OF_STREAM: &[u8; 4] = &[0, 0, 0, 0];

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3311".to_string();

    let socket = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    let (tx, rx) = mpsc::channel::<Message>(1);
    let (tracker_tx, tracker_rx) = mpsc::channel::<TrackerMessage>(1);
    let tracker_tx_one = tracker_tx.clone();

    tokio::spawn(async {
        TrackerActor::new(tracker_rx).run().await;
    });
    tokio::spawn(async move {
        let order_book_actor = OrderBookActor::new(rx, tracker_tx_one.clone(), 20.0);
        order_book_actor.run().await;
    });
    println!("Order book actor running now");

    while let Ok((mut stream, peer)) = socket.accept().await {
        println!("Incoming connection from: {}", peer.to_string());
        let tx_one = tx.clone();
        let tracker_tx_two = tracker_tx.clone();
        tokio::spawn(async move {
            println!("thread {} starting", peer.to_string());
            let (reader, mut writer) = stream.split();
            let mut buf_reader = BufReader::new(reader);
            let mut buf = vec![];

            loop {
                match buf_reader.read_to_end(&mut buf).await {
                    Ok(n) => {
                        if n == 0 {
                            println!("EOF received");
                            break;
                        }

                        let buf_string = String::from_utf8_lossy(&buf);
                        let data: Vec<String> = buf_string
                            .split("\0")
                            .map(|x| x.to_string().replace("\n", ""))
                            .collect();
                        println!("here is the data {:?}", data);
                        let command = data[0].clone();

                        match command.as_str() {
                            "zINSTREAM" => {
                                println!("file order command processed");
                                //let amount = data[1].parse::<f32>().unwrap();
                                let amount = 32.0;
                                let order_actor =
                                    FileOrder::new(data[4].clone(), amount, tx_one.clone());
                                println!("{}: {}", order_actor.ticker, order_actor.amount);
                                order_actor.send().await;
                            }
                            "zVERSION" => {
                                println!("get version command processed");
                                let get_actor = VersionTrackerActor {
                                    sender: tracker_tx_two.clone(),
                                };
                                let state = get_actor.send().await;
                                println!("sending back: {:?}", state);
                                writer.write_all(state.as_bytes()).await.unwrap();
                            }
                            "zPING" => {
                                println!("PING command processed");
                                writer.write_all(b"PONG\0").await.unwrap();
                            }
                            _ => {
                                panic!("{} command not supported", command);
                            }
                        }
                        buf.clear();
                    }
                    Err(e) => println!("Error receiving message: {}", e),
                }
            }

            println!("thread {} finishing", peer.to_string());
        });
    }
}
