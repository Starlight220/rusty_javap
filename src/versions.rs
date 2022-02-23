use crate::typedefs::*;
use crate::{ByteReader, Take};
use std::fmt::{Display, Formatter};
use crate::model::class::Version;

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
        Ok(version)
    }
}
