use crate::bytecode::reader::{ByteReader, Take};
use crate::bytecode::writer::{ByteWriter, Writeable};
use crate::w2;
use serde::{Deserialize, Serialize};

pub type LineNumberTable = Vec<LineNumberTableElement>;

#[derive(Debug, Deserialize, Serialize)]
pub struct LineNumberTableElement {
    pub start_pc: w2,
    pub line_number: w2,
}

impl Take<LineNumberTable> for ByteReader {
    fn take(self: &mut ByteReader) -> Result<LineNumberTable, String> {
        let line_number_table_length: w2 = self.take()?;
        let mut line_number_table: Vec<LineNumberTableElement> =
            Vec::with_capacity(line_number_table_length.into());
        for _ in 0..line_number_table_length {
            let start_pc: w2 = self.take()?;
            let line_number: w2 = self.take()?;
            line_number_table.push(LineNumberTableElement {
                start_pc,
                line_number,
            });
        }
        Ok(line_number_table)
    }
}

impl Writeable for LineNumberTable {
    fn write(self, writer: &mut ByteWriter) {
        writer.write(self.len() as w2);
        for LineNumberTableElement {
            start_pc,
            line_number,
        } in self
        {
            writer.write(start_pc);
            writer.write(line_number);
        }
    }
}
