use super::handler::{Callback};

pub mod tcp {
    use std::net::{TcpStream, Shutdown};
    use std::io::{Read, Write};

    pub struct Tcp {
        stream: Box<TcpStream>,
    }

    impl Tcp {
        fn new() -> Self {
            Tcp {
                stream: nil,
            }
        }

        fn connect(&mut self) -> std::io::Result<()> {
            self.stream = TcpStream::bind(addr.to_string())?;
            self.stream.write(format!("connect->{}", addr).as_bytes())?;
            Ok(())
        }

        fn disconnect(&mut self) -> std::io::Result<()> {
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
}
