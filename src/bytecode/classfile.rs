use crate::bytecode::attributes::UnresolvedAttribute;
use crate::bytecode::fields::UnresolvedField;
use crate::bytecode::interfaces::UnresolvedInterfaces;
use crate::bytecode::methods::UnresolvedMethod;
use crate::bytecode::reader::{ByteReader, Take};
use crate::bytecode::unresolved::Unresolved;
use crate::bytecode::writer::{ByteWriter, Writeable};
use crate::constant_pool::{Constant, ConstantPool, CpInfo, CpTag};
use crate::{w2, Class};

impl Take<Class> for ByteReader {
    fn take(&mut self) -> Result<Class, String> {
        let version = self.take()?;
        let constant_pool: ConstantPool = self.take()?;
        let access_flags = self.take()?;

        let this_class_index: w2 = self.take()?;
        let this_class = constant_pool.get_class_name(this_class_index)?;

        let super_class_index: w2 = self.take()?;
        let super_class = if super_class_index == 0 {
            Option::None
        } else {
            Option::Some(constant_pool.get_class_name(super_class_index)?)
        };

        let unresolved_interfaces: UnresolvedInterfaces = self.take()?;
        let interfaces = unresolved_interfaces.resolve(&constant_pool)?;

        let unresolved_fields: Vec<UnresolvedField> = self.take()?;
        let fields = unresolved_fields.resolve(&constant_pool)?.into();

        let unresolved_methods: Vec<UnresolvedMethod> = self.take()?;
        let methods = unresolved_methods.resolve(&constant_pool)?.into();

        let unresolved_attributes: Vec<UnresolvedAttribute> = self.take()?;
        let attributes = unresolved_attributes.resolve(&constant_pool)?;

        Ok(Class::new(
            version,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        ))
    }
}

impl Writeable for Class {
    fn write(self, writer: &mut ByteWriter) {
        let version = self.version;
        let mut constant_pool: ConstantPool = ConstantPool::new();
        let access_flags = self.access_flags;

        let this_class_index: w2 = {
            let class_name_index = constant_pool.push(Constant(
                CpTag::Utf8,
                CpInfo::Utf8 {
                    string: self.this_class,
                },
            ));
            constant_pool.push(Constant(
                CpTag::Class,
                CpInfo::Class {
                    name_index: class_name_index,
                },
            ))
        };

        let super_class_index: w2 = match self.super_class {
            Option::None => 0,
            Option::Some(class_name) => {
                let class_name_index =
                    constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string: class_name }));
                constant_pool.push(Constant(
                    CpTag::Class,
                    CpInfo::Class {
                        name_index: class_name_index,
                    },
                ))
            }
        };

        let unresolved_interfaces: UnresolvedInterfaces =
            Unresolved::unresolve(self.interfaces, &mut constant_pool);
        let unresolved_fields: Vec<UnresolvedField> =
            Unresolved::unresolve(self.fields, &mut constant_pool);
        let unresolved_methods: Vec<UnresolvedMethod> =
            Unresolved::unresolve(self.methods, &mut constant_pool);

        let unresolved_attributes: Vec<UnresolvedAttribute> =
            Unresolved::unresolve(self.attributes, &mut constant_pool);

        writer.write(version);
        writer.write(constant_pool);
        writer.write(access_flags);
        writer.write(this_class_index);
        writer.write(super_class_index);
        writer.write(unresolved_interfaces);
        writer.write(unresolved_fields);
        writer.write(unresolved_methods);
        writer.write(unresolved_attributes);
    }
}
