use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use tokio::net::TcpStream;

//const PING: &[u8; 6] = b"zPING\0";
//const VERSION: &[u8; 9] = b"zVERSION\0";
const INSTREAM: &[u8; 10] = b"zINSTREAM\0";
const END_OF_STREAM: &[u8; 4] = &[0, 0, 0, 0];
const DEFAULT_CHUNK_SIZE: usize = 4096;
const FILE_PATH: &str = "examples/eicar.com";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open(FILE_PATH)?;
    let mut buffer = vec![0; DEFAULT_CHUNK_SIZE];
    let stream = TcpStream::connect("127.0.0.1:3310").await?;

    loop {
        stream.writable().await?;

        // Try to write data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_write(INSTREAM) {
            Ok(n) => {
                // TODO: implement logging
                println!("write {} bytes", n);
                loop {
                    let len = file.read(&mut buffer[..])?;
                    if len != 0 {
                        stream.try_write(&(len as u32).to_be_bytes())?;
                        stream.try_write(&buffer[..len])?;
                    } else {
                        stream.try_write(END_OF_STREAM)?;
                        break;
                    }
                }
                break;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    let mut response = vec![0; 1024];

    loop {
        // Wait for the socket to be readable
        stream.readable().await?;

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read(&mut response) {
            Ok(n) => {
                println!("read {} bytes", n);
                response.truncate(n);
                break;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    println!("{:?}", str::from_utf8(&response).unwrap());

    Ok(())
}
