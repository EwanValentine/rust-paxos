use tokio::net::UdpSocket;

pub struct Udp {
    stream: Option<Box<UdpSocket>>,
}

impl Udp {
    fn new() -> Self {
        Udp {
            stream: Option::none,
        }
    }

    pub fn connect(&mut self, addr: String) -> std::io::Result<()> {
        self.stream = Option::from(
            Box::from(
                UdpSocket::bind(addr.to_string())?,
            ),
        );
        self.stream.broadcast();
        Ok(())
    }

    fn disconnect(&self) -> std::io::Result<()> {
        self.stream.leave_multicast_v4();
        Ok(())
    }

    fn listen(&self) -> std::io::Result<()> {
        self.stream.
        Ok(())
    }
}
