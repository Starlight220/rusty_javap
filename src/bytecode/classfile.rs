use crate::bytecode::reader::{ByteReader, Take};
use crate::{Class, w2};
use crate::bytecode::attributes::UnresolvedAttribute;
use crate::bytecode::fields::UnresolvedField;
use crate::bytecode::interfaces::UnresolvedInterfaces;
use crate::bytecode::methods::UnresolvedMethod;
use crate::bytecode::unresolved::Unresolved;
use crate::constant_pool::ConstantPool;

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
