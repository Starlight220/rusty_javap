use crate::typedefs::*;

pub struct ByteReader {
    buffer: Vec<w1>,
    ptr: usize,
}

pub trait ByteSize {
    fn read(bytes: &[w1]) -> Self;
    fn width() -> usize;
}

pub trait Take<T> {
    fn take(&mut self) -> Result<T, String>;
}

impl<Un: ByteSize> Take<Un> for ByteReader {
    fn take(&mut self) -> Result<Un, String> {
        let bytes = self.take_bytes(Un::width())?;
        Ok(Un::read(bytes))
    }
}

impl From<Vec<w1>> for ByteReader {
    fn from(buffer: Vec<w1>) -> Self {
        Self { buffer, ptr: 0 }
    }
}

impl ByteReader {
    ///
    ///```rust
    /// use rusty_javap::bytecode::reader::ByteReader;
    /// assert!(ByteReader::from(vec![]).is_empty());
    /// assert!(!ByteReader::from(vec![1]).is_empty());
    /// assert!(!ByteReader::from(vec![1;3]).is_empty());
    ///```
    ///```rust
    /// use rusty_javap::bytecode::reader::ByteReader;
    /// let mut reader = ByteReader::from(vec![1;12]);
    /// assert!(!reader.is_empty());
    /// let _ = reader.take_bytes(4);
    /// assert!(!reader.is_empty());
    /// let _ = reader.take_bytes(8);
    /// assert!(reader.is_empty());
    /// assert_eq!(reader.take_bytes(1), Err("Can't take 1 bytes from buffer containing 0 bytes!".to_string()));
    ///```
    ///```rust
    /// use rusty_javap::bytecode::reader::ByteReader;
    /// let mut reader = ByteReader::from(vec![1;12]);
    /// assert!(!reader.is_empty());
    /// let _ = reader.take_bytes(4);
    /// assert!(!reader.is_empty());
    /// assert_eq!(reader.take_bytes(10), Err("Can't take 10 bytes from buffer containing 8 bytes!".to_string()));
    ///```
    ///
    pub fn is_empty(&self) -> bool {
        self.ptr >= self.buffer.len()
    }

    pub fn deplete(mut self) -> Vec<w1> {
        self.take_bytes(usize::MAX)
            .expect("Assertion failed: ByteReader::deplete should never fail!")
            .to_vec()
    }

    /// Take [size] bytes from the buffer.
    /// Pass [usize::MAX] to get the whole buffer -- this cannot fail.
    ///
    ///```rust
    /// use rusty_javap::bytecode::reader::ByteReader;
    /// assert_eq!(ByteReader::from(vec![0; 12]).take_bytes(3), Ok(vec![0;3].as_slice()));
    /// assert_eq!(ByteReader::from(vec![]).take_bytes(0), Ok(vec![].as_slice()));
    /// assert_eq!(ByteReader::from(vec![1]).take_bytes(3), Err("Can't take 3 bytes from buffer containing 1 bytes!".to_string()));
    /// assert_eq!(ByteReader::from(vec![]).take_bytes(3), Err("Can't take 3 bytes from buffer containing 0 bytes!".to_string()));
    /// assert_eq!(ByteReader::from(vec![]).take_bytes(usize::MAX), Ok(vec![].as_slice()));
    /// assert_eq!(ByteReader::from(vec![0; 12]).take_bytes(usize::MAX), Ok(vec![0;12].as_slice()));
    ///```
    ///
    pub fn take_bytes(&mut self, size: usize) -> Result<&[w1], String> {
        let max_request = self.buffer.len() - self.ptr;
        let request = if size == usize::MAX {
            max_request
        } else {
            size
        };
        let _ = size;
        if request > max_request {
            return Err(format!(
                "Can't take {} bytes from buffer containing {} bytes!",
                request, max_request
            ));
        }
        let previous_ptr = self.ptr;
        self.ptr += request;
        let bytes = &self.buffer[previous_ptr..self.ptr];
        Ok(bytes)
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
