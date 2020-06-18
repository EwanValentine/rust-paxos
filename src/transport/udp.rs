use tokio::net::UdpSocket;
use crate::transport::transport::Adapter;
use crate::transport::handler::Callback;
use std::thread;
use std::io::ErrorKind;
use std::time::{Duration, Instant};

pub struct Udp {
    stream: UdpSocket,
}

impl Udp {
    fn connect(addr: String) -> Self {
        let stream = UdpSocket::bind(addr.to_string())?;
        stream.broadcast();
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

    fn listen(self, handler: &dyn Callback) -> std::io::Result<()> {
        let start = Instant::now();
        let mut buf = [0u8; 1024];

        while start.elapsed().as_secs() < 1 {
            let result = self.stream.recv(&mut buf);
            match result {
                Ok(num_bytes) => {
                    handler.callback(buf)
                },
                Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                    println!("Failure: {}", err)
                }
                _ => {}
            }

            thread::sleep(Duration::from_millis(5));
        }

        Ok(())
    }
}
