use crate::typedefs::*;

pub struct ByteReader {
    buffer: Vec<w1>,
    ptr: usize,
}

pub trait ByteSize {
    fn read(bytes: &[w1]) -> Self;
    fn width() -> usize;
}

impl ByteReader {
    pub fn take<Un: ByteSize>(&mut self) -> Un {
        let start = self.ptr;
        self.ptr += Un::width();
        return Un::read(&self.buffer[start..self.ptr]);
    }
}

impl From<Vec<w1>> for ByteReader {
    fn from(buffer: Vec<w1>) -> Self {
        return ByteReader { buffer, ptr: 0 };
    }
}

macro_rules! impl_read_for {
    ($t:ty, $width:expr) => {
        impl ByteSize for $t {
            fn read(bytes: &[w1]) -> $t {
                let mut u: $t = 0;
                for i in (0..$width).rev() {
                    u |= ((bytes[i] as $t) << 8 * ($width - 1 - i))
                }
                return u;
            }
            fn width() -> usize {
                return $width;
            }
        }
    };
}

impl_read_for!(w1, 1);
impl_read_for!(w2, 2);
impl_read_for!(w4, 4);
