use crate::transport::handler::Callback;
use tokio::net::ToSocketAddrs;

pub trait Adapter {
  fn disconnect(self) -> std::io::Result<()>;
  fn listen(self, handler: &dyn Callback) -> std::io::Result<()>;
}

#[derive(Debug)]
pub struct Transport {
  adapter: dyn Adapter
}

impl Transport {
  pub fn new(adapter: &impl Adapter) -> Box<Transport> {
    Box::from(Transport { adapter })
  }

  pub fn connect(self, addr: String) -> std::io::Result<()> {
    self.adapter.connect(addr)
  }

  pub fn connect_with_stream<S>(self, stream: S) -> std::io::Result<(S)> {
    self.adapter.connect_with_stream(stream)
  }

  pub fn disconnect(self) -> std::io::Result<()> {
    self.adapter.disconnect()
  }

  pub fn listen(self, handler: &dyn Callback) -> std::io::Result<()> {
    self.adapter.listen(handler)
  }
}
