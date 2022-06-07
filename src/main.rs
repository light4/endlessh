use std::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:2222").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    loop {
        sleep(Duration::from_secs(10)).await;
        socket.writable().await.unwrap();
        let s = format!("{:x}\r\n", fastrand::u32(..));
        match socket.try_write(s.as_bytes()) {
            Ok(_) => {
                dbg!("ok");
                continue;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                dbg!("block");
                continue;
            }
            Err(e) => {
                dbg!(e);
                break;
            }
        }
    }
}
