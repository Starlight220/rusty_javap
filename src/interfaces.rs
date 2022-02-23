use crate::constant_pool::ConstantPool;
use crate::{w2, ByteReader, Take, Unresolved};
use std::fmt::{Display, Formatter};

pub type UnresolvedInterfaces = Vec<w2>;

impl Unresolved for UnresolvedInterfaces {
    type Resolved = Interfaces;
    type NeededToResolve = ConstantPool;

    fn resolve(self, constant_pool: &Self::NeededToResolve) -> Result<Self::Resolved, String> {
        let mut interfaces = vec![];

        for index in self {
            interfaces.push(constant_pool.get_class_name(index)?);
        }

        Ok(Interfaces { pool: interfaces })
    }
}


impl Take<UnresolvedInterfaces> for ByteReader {
    fn take(&mut self) -> Result<UnresolvedInterfaces, String> {
        let interface_count: w2 = self.take()?;
        let mut interfaces = vec![];
        for _ in 0..interface_count {
            interfaces.push(self.take()?)
        }
        return Ok(interfaces);
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
