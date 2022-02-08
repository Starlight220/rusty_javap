use std::fmt::{Display, Formatter};
use crate::constant_pool::ConstantPool;
use crate::{ByteReader, Take, w2};

#[derive(Debug)]
pub struct Interfaces {
    pool: Vec<String>
}

impl Interfaces {
    pub fn resolve_all(constant_pool: &ConstantPool, indexes: Vec<w2>) -> Result<Interfaces,
        String> {
        let mut interfaces = vec![];

        for index in indexes {
            interfaces.push(constant_pool.get_class_name(index)?);
        }

        Ok(Interfaces {
            pool: interfaces
        })
    }
}

impl Take<Vec<w2>> for ByteReader {
    fn take(&mut self) -> Result<Vec<w2>, String> {
        let interface_count: w2 = self.take()?;
        let mut interfaces = vec![];
        for _ in 0..interface_count {
            interfaces.push(self.take()?)
        }
        return Ok(interfaces)
    }
}

impl Display for Interfaces {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.pool.len() > 0 {
            writeln!(f, "Interfaces:")?;
            for interface in &self.pool {
                writeln!(f, "\t- {}", interface)?;
            }
        } else {
            writeln!(f, "Interfaces: None")?;
        }
        write!(f, "")
    }
}
