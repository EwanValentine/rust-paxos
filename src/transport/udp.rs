pub mod udp {
    use std::net::UdpSocket;

    pub struct Udp {
        stream: Box<UdpSocket>,
    }

    impl Udp {
        fn new() -> Self {
            Udp { stream: nil }
        }

        fn connect(&mut self, addr: String) -> std::io::Result<()> {
            self.stream = Box::from(UdpSocket::bind(addr.to_string())?);
            self.stream.broadcast();
            Ok(())
        }

        fn disconnect(&self) -> std::io::Result<()> {
            self.stream.leave_multicast_v4();
            Ok(())
        }

        fn listen() -> std::io::Result<()> {
            Ok(())
        }
    }
}
