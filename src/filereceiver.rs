use std::path::PathBuf;
use tokio::io::{AsyncWriteExt, ErrorKind};
use tokio::net::TcpListener;

const END_OF_STREAM: &[u8; 4] = &[0, 0, 0, 0];

pub struct FileReceiver {
    pub socket: TcpListener,
    pub filepath: PathBuf,
}

impl FileReceiver {
    pub fn new(socket: TcpListener, tempdir: PathBuf) -> Self {
        Self {
            socket,
            filepath: tempdir,
        }
    }

    pub async fn run(self) {
        println!("FileReceiver is running");
        while let Ok((mut stream, peer)) = self.socket.accept().await {
            let mut intf = self.filepath.clone();
            tokio::spawn(async move {
                let (reader, mut writer) = stream.split();
                let bytes_to_read_per_attempt = 128;
                let mut read_attempt_nr = 0;
                let mut command = "zINSTREAM".to_string();
                intf.push(format!("scan_it_{}", peer.port().to_string()));
                let mut file = tokio::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(intf)
                    .await
                    .unwrap();

                loop {
                    read_attempt_nr += 1;
                    let mut cur_buffer = vec![0; bytes_to_read_per_attempt];

                    match reader.try_read(&mut cur_buffer) {
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
                                command = data[0].clone();
                            }

                            if command.as_str() == "zINSTREAM" {
                                let last4 = cur_buffer.as_slice()[cur_buffer.len() - 4..].to_vec();
                                cur_buffer.truncate(nr);
                                match file.write_all(&cur_buffer).await {
                                    Ok(()) => {
                                        // TODO refactor with checksums
                                        //
                                        println!("chunk {} saved", read_attempt_nr);
                                    }
                                    Err(e) => println!("Error saving file: {}", e),
                                }

                                if last4 == END_OF_STREAM {
                                    println!("0000 EOF received");
                                    // !!! No breaks needed !!!
                                    //break;
                                }
                            }
                        }
                        Err(ref e)
                            if e.kind() == ErrorKind::WouldBlock
                                || e.kind() == ErrorKind::TimedOut =>
                        {
                            break;
                        }
                        Err(e) => {
                            println!("Error receiving message: {}", e);
                            break;
                        }
                    };
                }
                match command.as_str() {
                    "zINSTREAM" => {
                        let response = format!("stream: {} FOUND\0", peer.port().to_string());
                        writer.write_all(response.as_bytes()).await.unwrap();
                        // TODO: refactor with queues
                        //
                    }
                    "zVERSION" => {
                        writer
                            .write_all(b"ClamAV 1.0.6 compatible\0")
                            .await
                            .unwrap();
                    }
                    "zPING" => {
                        writer.write_all(b"PONG\0").await.unwrap();
                    }
                    _ => {
                        panic!("{} command not supported", command);
                    }
                }
                writer.flush().await.unwrap();
            });
        }
    }
}
