use std::path::PathBuf;
use tokio::io::{AsyncWriteExt, ErrorKind};
use tokio::net::TcpListener;

use super::END_OF_STREAM;

pub struct FileReceiver {
    pub socket: TcpListener,
    pub filepath: PathBuf,
}

impl FileReceiver {
    pub fn new(socket: TcpListener) -> Self {
        // TODO: filepath from env
        //
        Self {
            socket,
            filepath: PathBuf::from("/tmp"),
        }
    }

    fn handle_filestream(&mut self) {
        println!("got file!");
    }

    pub async fn run(self) {
        println!("FileReceiver is running");
        while let Ok((mut stream, peer)) = self.socket.accept().await {
            println!("Incoming connection from: {}", peer.to_string());
            tokio::spawn(async move {
                println!("thread {} starting", peer.to_string());
                let (reader, mut writer) = stream.split();
                let mut total_bytes_read = vec![];
                let bytes_to_read_per_attempt = 1024;
                let mut read_attempt_nr = 0;
                let mut command = "zINSTREAM".to_string();

                loop {
                    read_attempt_nr += 1;
                    // println!("Read attempt nr {read_attempt_nr}");
                    let mut cur_buffer = vec![0; bytes_to_read_per_attempt];

                    let nr = match reader.try_read(&mut cur_buffer) {
                        Ok(nr) => {
                            if nr == 0 {
                                println!("EOF received");
                                break;
                            }

                            if read_attempt_nr == 1 {
                                let first10 = cur_buffer.as_slice()[0..9].to_vec();
                                let buf_string = String::from_utf8_lossy(&first10);
                                let data: Vec<String> =
                                    buf_string.split("\0").map(|x| x.to_string()).collect();
                                //println!("first attempt data {:?}", data[0]);
                                command = data[0].clone();
                            }

                            let last4 = cur_buffer.as_slice()[cur_buffer.len() - 4..].to_vec();
                            if last4 == END_OF_STREAM {
                                println!("0000 EOF received");
                                // TODO: refactor with queues
                                //
                                match command.as_str() {
                                    "zINSTREAM" => {
                                        println!("file order command processed");
                                        // !!! Fictive response !!!
                                        //
                                        //self.handle_filestream();
                                        writer
                                            .write_all(b"stream: Win.Test.EICAR_HDB-1 FOUND\0")
                                            .await
                                            .unwrap();
                                    }
                                    "zVERSION" => {
                                        //println!("get version command processed");
                                        writer
                                            .write_all(b"ClamAV 1.0.6 compatible\0")
                                            .await
                                            .unwrap();
                                    }
                                    "zPING" => {
                                        println!("PING command processed");
                                        writer.write_all(b"PONG\0").await.unwrap();
                                    }
                                    _ => {
                                        panic!("{} command not supported", command);
                                    }
                                }
                                writer.flush().await.unwrap();

                                break;
                            }
                            nr
                        }
                        Err(ref e)
                            if e.kind() == ErrorKind::WouldBlock
                                || e.kind() == ErrorKind::TimedOut =>
                        {
                            //println!("Read attempt timed out");
                            break;
                        }
                        Err(e) => {
                            println!("Error receiving message: {}", e);
                            break;
                        }
                    };
                    cur_buffer.truncate(nr);

                    // TODO: save file and task in queue
                    //
                    total_bytes_read.append(&mut cur_buffer);
                }

                println!("thread {} finishing", peer.to_string());
            });
        }
    }
}
