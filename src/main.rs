use anyhow::Result;
use std::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info};
use tracing_subscriber;

const ADDR: &'static str = "0.0.0.0:2222";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind(ADDR).await?;
    info!("server listening: {}", ADDR);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process(socket).await {
                error!("process error: {:#?}", e);
            }
        });
    }
}

async fn process(socket: TcpStream) -> Result<()> {
    let addr = socket.peer_addr()?;
    info!("connect from: {}", addr);

    loop {
        sleep(Duration::from_secs(10)).await;
        socket.writable().await?;
        let s = format!("{:x}\r\n", fastrand::u32(..));
        match socket.try_write(s.as_bytes()) {
            Ok(_) => {
                debug!("write to: {}", addr);
                continue;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                debug!("block at: {}", addr);
                continue;
            }
            Err(e) => {
                debug!("error at {}: {:#?}", addr, e);
                break;
            }
        }
    }

    Ok(())
}
