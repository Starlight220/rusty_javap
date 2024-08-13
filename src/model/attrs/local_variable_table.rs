use serde::{Deserialize, Serialize};

use crate::w2;

pub type LocalVariableTable = Vec<LocalVariableTableElement>;

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalVariableTableElement {
    pub start_pc: w2,
    pub length: w2,
    pub name: String,
    pub descriptor: String,
    pub index: w2,
}
