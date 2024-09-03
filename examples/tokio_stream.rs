use std::error::Error;
use std::io;
use tokio::net::TcpStream;

//const PING: &[u8; 6] = b"zPING\0";
const VERSION: &[u8; 9] = b"zVERSION\0";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let stream = TcpStream::connect("127.0.0.1:3310").await?;

    loop {
        // Wait for the socket to be writable
        stream.writable().await?;

        // Try to write data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_write(VERSION) {
            Ok(n) => {
                println!("write {} bytes", n);
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    //let mut response = Vec::new();
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
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    println!("{:?}", response);

    Ok(())
}
