use crate::constant_pool::ConstantPool;
use crate::model::interface::Interface;
use crate::w2;
use crate::bytecode::reader::{ByteReader, Take};
use crate::bytecode::unresolved::Unresolved;

pub type UnresolvedInterfaces = Vec<w2>;

impl Unresolved for UnresolvedInterfaces {
    type Resolved = Vec<Interface>;
    type NeededToResolve = ConstantPool;

    fn resolve(self, constant_pool: &Self::NeededToResolve) -> Result<Self::Resolved, String> {
        let mut interfaces = vec![];

        for index in self {
            interfaces.push(Interface::new(constant_pool.get_class_name(index)?));
        }

        Ok(interfaces)
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
