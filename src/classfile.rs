use crate::access::ClassAccessModifier;
use crate::attributes::{Attributes, UnresolvedAttribute};
use crate::constant_pool::ConstantPool;
use crate::fields::{Fields, UnresolvedField};
use crate::interfaces::{Interfaces, UnresolvedInterfaces};
use crate::methods::{Methods, UnresolvedMethod};
use crate::versions::Version;
use crate::{w2, ByteReader, Take, Unresolved};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Class {
    version: Version,
    constant_pool: ConstantPool,
    access_flags: Vec<ClassAccessModifier>,
    this_class: String,
    super_class: Option<String>,
    interfaces: Interfaces,
    fields: Fields,
    methods: Methods,
    attributes: Attributes,
}

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

        Ok(Class {
            version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.version)?;
        writeln!(f, "{}", self.constant_pool)?;
        writeln!(f, "{:?}", self.access_flags)?; // TODO
        writeln!(f, "Class: {}", self.this_class)?;
        writeln!(
            f,
            "Superclass: {}",
            self.super_class.as_ref().unwrap_or(&"None".to_string())
        )?;
        writeln!(f, "{}", self.interfaces)?;
        writeln!(f, "{}", self.fields)?;
        writeln!(f, "{}", self.methods)?;
        writeln!(f, "{}", self.attributes)?;
        write!(f, "")
    }
}
