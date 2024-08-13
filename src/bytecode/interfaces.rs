use crate::bytecode::reader::{ByteReader, Take};
use crate::bytecode::unresolved::Unresolved;
use crate::bytecode::writer::{ByteWriter, Writeable};
use crate::constant_pool::{Constant, ConstantPool, CpInfo, CpTag};
use crate::model::interface::Interface;
use crate::w2;

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

    fn unresolve(resolved: Self::Resolved, constant_pool: &mut Self::NeededToResolve) -> Self {
        let mut unresolved = UnresolvedInterfaces::default();
        for Interface(interface) in resolved {
            let class_name_index =
                constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string: interface }));

            let class_index = constant_pool.push(Constant(
                CpTag::Class,
                CpInfo::Class {
                    name_index: class_name_index,
                },
            ));

            unresolved.push(class_index);
        }
        unresolved
    }
}

impl Take<UnresolvedInterfaces> for ByteReader {
    fn take(&mut self) -> Result<UnresolvedInterfaces, String> {
        let interface_count: w2 = self.take()?;
        let mut interfaces = vec![];
        for _ in 0..interface_count {
            interfaces.push(self.take()?)
        }
        Ok(interfaces)
    }
}

impl Writeable for UnresolvedInterfaces {
    fn write(self, writer: &mut ByteWriter) {
        writer.write(self.len() as w2);
        for interface in self {
            writer.write(interface)
        }
    }
}
