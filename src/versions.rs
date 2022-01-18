use crate::typedefs::*;
use crate::{ByteReader, Take};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Version {
    magic: w4,
    major: w2,
    minor: w2,
}

impl Take<Version> for ByteReader {
    fn take(&mut self) -> Result<Version, String> {
        let magic: w4 = self.take()?;
        let minor: w2 = self.take()?;
        let major: w2 = self.take()?;

        if magic != 0xCAFEBABE {
            return Err(format!("Wrong 'magic' field: `{:#X}`!", magic));
        }

        let version = Version {
            magic,
            major,
            minor,
        };
        println!("{}", version);
        Ok(version)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "magic: {:#X}\nmajor: {}\nminor: {}",
            self.magic, self.major, self.minor
        )
    }
}
