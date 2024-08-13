use serde::{Deserialize, Serialize};

use crate::model::attrs::Attribute;
use crate::{w1, w2};

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    pub max_stack: w2,
    pub max_locals: w2,
    /// Len: w4
    pub code: Vec<w1>,
    /// Len: w2
    pub exception_table: Vec<ExceptionTableElement>,
    /// Len: w2
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionTableElement {
    pub start_pc: w2,
    pub end_pc: w2,
    pub handler_pc: w2,
    pub catch_type: Option<String>,
}
