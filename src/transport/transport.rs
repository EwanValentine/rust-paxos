use crate::transport::handler::Callback;

pub trait Adapter {
  fn connect(self, addr: String) -> std::io::Result<()>;
  fn disconnect(self) -> std::io::Result<()>;
  fn listen(self, handler: &dyn Callback) -> std::io::Result<()>;
}

pub struct Transport<'a> {
  adapter: Box<dyn Adapter>,
}

impl Transport<'_> {
  fn new(adapter: Box<dyn Adapter>) -> Transport {
    Transport {
      adapter,
    }
  }

  fn connect(self, addr: String) -> std::io::Result<()> {
    self.adapter.connect(addr)
  }

  fn disconnect(self) -> std::io::Result<()> {
    self.adapter.disconnect()
  }

  fn listen(self, handler: &dyn Callback) -> std::io::Result<()> {
    self.adapter.listen(handler)
  }
}
