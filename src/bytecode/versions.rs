use crate::bytecode::reader::{ByteReader, Take};
use crate::bytecode::writer::{ByteWriter, Writeable};
use crate::model::class::Version;
use crate::typedefs::*;

impl Take<Version> for ByteReader {
    fn take(&mut self) -> Result<Version, String> {
        let magic: w4 = self.take()?;
        let minor: w2 = self.take()?;
        let major: w2 = self.take()?;

        if magic != 0xCAFEBABE {
            return Err(format!("Wrong 'magic' field: `{:#X}`!", magic));
        }

        let version = Version::new(magic, major, minor);
        Ok(version)
    }
}

impl Writeable for Version {
    fn write(&self, writer: &mut ByteWriter) {
        let Version {magic, major, minor} = self;
        writer.write(magic);
        writer.write(major);
        writer.write(minor);
    }
}
