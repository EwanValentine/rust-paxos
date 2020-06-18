use std::io::Bytes;

pub trait Callback {
    fn callback(&self, data: [u8; 1024]);
}

impl <T: Fn()> Callback for T {
    fn callback(&self, data: [u8; 1024]) {
        self(data)
    }
}