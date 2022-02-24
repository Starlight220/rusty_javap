use serde::{Deserialize, Serialize};

use crate::{w4, w8};

#[derive(Debug, Serialize, Deserialize)]
pub enum ConstantValue {
    Integer(w4),
    Long(w8),
    Float(f32),
    Double(f64),
    String(String),
}
