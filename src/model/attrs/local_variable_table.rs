use crate::bytecode::reader::{ByteReader, Take};
use crate::bytecode::writer::ByteWriter;
use crate::constant_pool::{Constant, ConstantPool, CpInfo, CpTag};
use crate::w2;
use serde::{Deserialize, Serialize};

pub type LocalVariableTable = Vec<LocalVariableTableElement>;

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalVariableTableElement {
    pub start_pc: w2,
    pub length: w2,
    pub name: String,
    pub descriptor: String,
    pub index: w2,
}

pub fn parse_local_variable_table(
    bytes: &mut ByteReader,
    constant_pool: &ConstantPool,
) -> Result<LocalVariableTable, String> {
    let local_variable_table_length: w2 = bytes.take()?;
    let mut local_variable_table: Vec<LocalVariableTableElement> =
        Vec::with_capacity(local_variable_table_length.into());
    for _ in 0..local_variable_table_length {
        let start_pc: w2 = bytes.take()?;
        let length: w2 = bytes.take()?;
        let name_index: w2 = bytes.take()?;
        let name = constant_pool.get_utf8(name_index)?;
        let descriptor_index: w2 = bytes.take()?;
        let descriptor = constant_pool.get_utf8(descriptor_index)?;
        let index: w2 = bytes.take()?;
        local_variable_table.push(LocalVariableTableElement {
            start_pc,
            length,
            name,
            descriptor,
            index,
        });
    }
    Ok(local_variable_table)
}

pub fn write_local_variable_table(
    line_number_table: Vec<LocalVariableTableElement>,
    constant_pool: &mut ConstantPool,
) -> Vec<u8> {
    let mut writer = ByteWriter::new();
    writer.write(line_number_table.len() as w2);
    for LocalVariableTableElement {
        start_pc,
        length,
        name,
        descriptor,
        index,
    } in line_number_table
    {
        writer.write(start_pc);
        writer.write(length);
        let name_index = constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string: name }));
        writer.write(name_index);
        let descriptor_index =
            constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string: descriptor }));
        writer.write(descriptor_index);
        writer.write(index);
    }

    writer.into()
}
