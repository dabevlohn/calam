use std::path::PathBuf;
use std::str;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

//const PING: &[u8; 6] = b"zPING\0";
//const VERSION: &[u8; 9] = b"zVERSION\0";
const INSTREAM: &[u8; 10] = b"zINSTREAM\0";
const END_OF_STREAM: &[u8; 4] = &[0, 0, 0, 0];

pub struct StreamSender {
    pub stream: TcpStream,
    pub filepath: PathBuf,
}

impl StreamSender {
    pub fn new(stream: TcpStream, filepath: PathBuf) -> Self {
        Self { stream, filepath }
    }
    // TODO: refactor as tokio async function in case of file handling
    //
    pub async fn clam_scan(mut self) {
        loop {
            self.stream.writable().await.unwrap();

            // Try to write data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match self.stream.try_write(INSTREAM) {
                Ok(_n) => {
                    // TODO: implement logging
                    //
                    let mut buffer = vec![0; 128];
                    let mut file = tokio::fs::OpenOptions::new()
                        .write(false)
                        .read(true)
                        .create(false)
                        .open(self.filepath)
                        .await
                        .unwrap();

                    loop {
                        let len = file.read(&mut buffer[..]).await.unwrap();
                        if len == 0 {
                            self.stream.try_write(END_OF_STREAM).unwrap();
                            self.stream.flush().await.unwrap();
                            break;
                        } else {
                            self.stream
                                .write_all(&(len as u32).to_be_bytes())
                                .await
                                .unwrap();
                            self.stream.write_all(&buffer[..len]).await.unwrap();
                        }
                        println!("write {} bytes", len);
                    }

                    break;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    println!("Error sending file {:?}", e);
                }
                Err(e) => {
                    println!("Error sending file {:?}", e);
                }
            }
        }

        let mut response = vec![0; 1024];

        loop {
            // Wait for the socket to be readable
            self.stream.readable().await.unwrap();

            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match self.stream.try_read(&mut response) {
                Ok(n) => {
                    println!("read {} bytes", n);
                    response.truncate(n);
                    break;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    println!("Error reading response {:?}", e);
                }
            }
        }

        println!("{:?}", str::from_utf8(&response).unwrap());
    }
}
