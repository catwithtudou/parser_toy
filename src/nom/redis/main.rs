

use std::error::Error;

use bytes::{BufMut, BytesMut};
use structopt::StructOpt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::nom::redis::{command, resp};


pub async fn redis_cli() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    info!("redis-cli start");


    let mut stream = TcpStream::connect("127.0.0.1:6379").await?;
    let mut buf = [0u8; 1024];
    let mut resp = BytesMut::with_capacity(1024);

    let (mut reader, mut writer) = stream.split();
    // ping
    // writer.write(b"*1\r\n$4\r\nPING\r\n").await?;

    // command
    let com = command::Commands::from_args();
    writer.write(&com.to_bytes()).await?;

    // read
    let n = reader.read(&mut buf).await?;
    resp.put(&buf[0..n]);
    let reply = resp::Resp::from_resp(&resp);
    println!("{}", reply);
    Ok(())
}
