use async_std::net::ToSocketAddrs;

// Connection isn't included here, because we connect, return an instance,
// then passed the connected instance into the server. So by that point,
// we only need these methods.
//
pub trait Adapter {
  fn connect<T>(addr: String) -> std::io::Result<(T)>;
  fn connect_with_stream<T, S>(stream: S) -> std::io::Result<(T)>;
  fn disconnect(self) -> std::io::Result<()>;
  fn listen(&mut self, handler: fn(data: [u8; 1024])) -> std::io::Result<()>;
  fn write(&mut self, message: &[u8]) -> std::io::Result<()>;
}

#[derive(Debug)]
pub struct Transport<'a> {
  adapter: &'a Adapter
}

impl<'a> Transport<'a> {
  pub fn new(adapter: &impl Adapter) -> Box<Transport> {
    Box::from(Transport { adapter })
  }

  pub fn connect<T>(self, addr: String) -> std::io::Result<(T)> {
    self.adapter.connect(addr);
    Ok(self)
  }

  pub fn connect_with_stream<T, S>(self, stream: S) -> std::io::Result<(T)> {
    self.adapter.connect_with_stream(stream);
    Ok(self)
  }

  pub fn disconnect(self) -> std::io::Result<()> {
    self.adapter.disconnect()
  }

  pub fn listen(&mut self, handler: fn(data: [u8; 1024])) -> std::io::Result<()> {
    self.adapter.listen(handler)
  }

  pub fn write(&mut self, message: &[u8]) -> std::io::Result<()> {
    self.adapter.write(message);
    Ok(())
  }
}
