use serde::{Deserialize, Serialize};

use crate::w2;

pub type LineNumberTable = Vec<LineNumberTableElement>;

#[derive(Debug, Deserialize, Serialize)]
pub struct LineNumberTableElement {
    pub start_pc: w2,
    pub line_number: w2,
}
