use std::path::Path;
use tokio::{
    fs::File,
    io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
};

use tokio_stream::{Stream, StreamExt};

use super::{IoResult, DEFAULT_CHUNK_SIZE};

async fn save_stream<
    S: Stream<Item = Result<bytes::Bytes, std::io::Error>>,
    RW: AsyncRead + AsyncWrite + Unpin,
>(
    input_stream: S,
    chunk_size: Option<usize>,
    mut output_stream: RW,
) -> IoResult {
    let chunk_size = chunk_size
        .unwrap_or(DEFAULT_CHUNK_SIZE)
        .min(u32::MAX as usize);

    let mut input_stream = std::pin::pin!(input_stream);

    while let Some(bytes) = input_stream.next().await {
        let bytes = bytes?;
        let bytes = bytes.as_ref();
        for chunk in bytes.chunks(chunk_size) {
            let len = chunk.len();
            //output_stream.write_all(&(len as u32).to_be_bytes()).await?;
            //output_stream.write_all(chunk).await?;
        }
    }

    Ok(())
}
