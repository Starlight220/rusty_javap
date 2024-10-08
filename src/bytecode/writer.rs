use crate::typedefs::*;

pub struct ByteWriter {
    buffer: Vec<w1>,
}

impl ByteWriter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { buffer: vec![] }
    }
    pub fn write_byte(&mut self, byte: w1) {
        self.buffer.push(byte)
    }

    pub fn write<W: Writeable>(&mut self, data: W) {
        data.write(self)
    }
}

pub trait Writeable {
    fn write(self, writer: &mut ByteWriter);
}

impl From<ByteWriter> for Vec<w1> {
    fn from(val: ByteWriter) -> Self {
        val.buffer
    }
}

macro_rules! impl_writeable_for {
    ($t:ty) => {
        impl Writeable for $t {
            fn write(self, writer: &mut ByteWriter) {
                for byte in self.to_be_bytes() {
                    writer.write_byte(byte);
                }
            }
        }
    };
}

impl_writeable_for!(w1);
impl_writeable_for!(w2);
impl_writeable_for!(w4);
