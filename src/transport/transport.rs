pub mod transport {
  pub trait Adapter {
    fn connect(self, addr: String) -> std::io::Result<()>;
    fn disconnect(self) -> std::io::Result<()>;
    fn listen(self) -> std::io::Result<()>;
  }

  pub struct Transport<'a> {
    adapter: &'a dyn Adapter,
  }

  impl Transport {
    fn new(adapter: &impl Adapter) -> Transport {
      Transport {
        adapter,
      }
    }

    fn connect(self, addr: String) -> std::io::Result<()> {
      self.adapter.connect(addr: String)
    }

    fn disconnect(self) -> std::io::Result<()> {
      self.adapter.disconnect()
    }

    fn listen(self) -> std::io::Result<()> {
      self.adapter.listen()
    }
  }
}
