use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use crate::transport::transport::Adapter;
use std::time::{Duration, Instant};

pub struct Tcp<'a> {
    stream: &'a TcpStream,
}

impl<'a> Adapter for Tcp<'a> {

    fn connect(addr: String) -> Self {
        let stream = TcpStream::connect(addr.to_string());
        let conn = match stream {
            Ok(mut t) => {
                t.write(format!("connect->{}", addr).as_bytes());
                Self { stream: &(t) }
            },
            Err(e) => panic!(e),
        };

        conn
    }

    fn connect_with_stream(stream: &'a TcpStream) -> Self {
        Self { stream }
    }

    fn disconnect(self) -> std::io::Result<()> {
        self.stream.shutdown(Shutdown::Both)?;
        Ok(())
    }

    fn listen(&mut self, handler: fn(data: [u8; 1024])) -> std::io::Result<()> {
        let start = Instant::now();
        while start.elapsed().as_secs() < 1 {
            let mut buffer = [0u8; 1024];
            self.stream.read(&mut buffer).unwrap();
            handler(buffer);
        }
        Ok(())
    }

    fn write(&mut self, message: &[u8]) -> std::io::Result<()> {
        let res = self.stream.write(message);

        // Should probably return some result here...
        Ok(())
    }
}
