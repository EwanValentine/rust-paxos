use tokio::net::UdpSocket;
use crate::transport::transport::Adapter;
use std::thread;
use std::io::ErrorKind;
use std::time::{Duration, Instant};

pub struct Udp {
    stream: UdpSocket,
}

impl Udp {
    async fn connect(addr: String) -> Self {
        let stream = UdpSocket::bind(&addr).await;
        Udp { stream }
    }

    fn connect_with_stream(stream: UdpSocket) -> Self {
        Udp { stream }
    }
}

impl Adapter for Udp {
    fn disconnect(self) -> std::io::Result<()> {
        Ok(())
    }

    fn listen(&mut self, handler: fn(data: [u8; 1024])) -> std::io::Result<()> {
        let start = Instant::now();
        while start.elapsed().as_secs() < 1 {
            let mut buf = [0u8; 1024];
            let data = match self.stream.recv_from(&mut buf) {
                Ok(rec) => {
                    handler(rec);
                },
                Err(_) => {},
            };

            thread::sleep(Duration::from_millis(5));
        }

        Ok(())
    }

    fn write(&mut self, message: &[u8]) -> std::io::Result<()> {
        Ok(())
    }
}
