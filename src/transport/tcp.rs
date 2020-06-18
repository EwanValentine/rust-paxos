use super::handler::{Callback};

use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use crate::transport::transport::Adapter;

pub struct Tcp<'a> {
    stream: &'a TcpStream,
}

impl Tcp<'_> {
    pub fn connect(addr: String) -> Self {
        let stream = TcpStream::bind(addr.to_string())?;
        stream.write(format!("connect->{}", addr).as_bytes())?;
        Self { stream }
    }

    pub fn connect_with_stream(stream: &TcpStream) -> Self {
        Self { stream }
    }
}

impl Adapter for Tcp<'_> {
    fn disconnect(self) -> std::io::Result<()> {
        self.stream.shutdown(Shutdown::Both)?;
        Ok(())
    }

    fn listen(self, handler: &dyn Callback) -> std::io::Result<()> {
        for stream in self.stream.incoming() {
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();
            let msg = String::from_utf8_lossy(&buffer[..]).into_owned();
            println!("Message: {}", msg);
            handler(msg);
        }
        Ok(())
    }
}