use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;

//const PING: &[u8; 6] = b"zPING\0";
//const VERSION: &[u8; 9] = b"zVERSION\0";
const INSTREAM: &[u8; 10] = b"zINSTREAM\0";
const END_OF_STREAM: &[u8; 4] = &[0, 0, 0, 0];
const DEFAULT_CHUNK_SIZE: usize = 4096;
const FILE_PATH: &str = "examples/eicar.com";

fn main() -> std::io::Result<()> {
    let mut file = File::open(FILE_PATH)?;
    let mut stream = TcpStream::connect("127.0.0.1:3310")?;
    stream.write_all(INSTREAM)?;

    let mut buffer = vec![0; DEFAULT_CHUNK_SIZE];
    loop {
        let len = file.read(&mut buffer[..])?;
        if len != 0 {
            stream.write_all(&(len as u32).to_be_bytes())?;
            stream.write_all(&buffer[..len])?;
        } else {
            stream.write_all(END_OF_STREAM)?;
            stream.flush()?;
            break;
        }
    }

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;
    //Ok(response)
    //stream.write(&[1])?;
    //stream.write_all(PING)?;
    //let resp = stream.read(&mut [0; 128])?;
    println!("{:?}", response);
    Ok(())
}
