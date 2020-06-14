use std::net::UdpSocket;

pub mod udp {
    pub struct Udp {}

    impl Udp {
        fn new() -> Udp {
            Udp {}
        }

        fn connect(addr: String) -> std::io::Result<()> {
            let mut stream = UdpStream::bind(addr.to_string())?;
            stream.write(format!("connect->{}", addr).as_bytes())?;
        }

        fn disconnect() -> std::io::Result<()> {}

        fn listen() -> std::io::Result<()> {}
    }
}
