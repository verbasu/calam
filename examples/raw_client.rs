use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:3310").await?;
    let (reader, mut writer) = stream.split();
    println!("stream starting");

    writer.write_all(b"zVERSION\0").await?;
    //writer
    //    .write_all(b"BUY;8.0;BYND\0BUY;9.0;PLTR\0BUY;9.0;PLTR\0GET\0")
    //    .await?;
    println!("sent data");

    let mut buf_reader = BufReader::new(reader);
    let mut buf = vec![];
    println!("reading data");
    let _ = buf_reader.read_until(b'\n', &mut buf).await.unwrap();
    let state_string = String::from_utf8_lossy(&buf);
    println!("{}", state_string);
    Ok(())
}
