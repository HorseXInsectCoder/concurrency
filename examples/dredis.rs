use anyhow::Result;
use std::{io, net::SocketAddr};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // build a listener
    let addr = "0.0.0.0:6379";

    // tokio 下的 TcpListener
    let listener = TcpListener::bind(addr).await?;
    info!("Dredis: Listening on: {}", addr);

    loop {
        // raddr for remote addr
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from: {}", raddr);

        // 生成线程处理 task
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, raddr).await {
                warn!("Error processing conn with {}: {:?}", raddr, e);
            }
        });
    }
}

async fn process_redis_conn(mut stream: TcpStream, raddr: SocketAddr) -> Result<()> {
    loop {
        // 好的习惯是确保 stream 是 readable 的
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        // 如果可读，那就去读数据
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                // 假设目前的内容都是可以转为 String 的
                let line = String::from_utf8_lossy(&buf);
                info!("{:?}", line);
                stream.write_all(b"+OK\r\n").await?; // redis 的文档建议返回 +OK\r\n
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    warn!("Connection {} closed!", raddr);
    Ok(())
}
