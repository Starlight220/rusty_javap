use serde::{Serialize, Deserialize};

use crate::{w4, w8};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum ConstantValue {
    Integer(w4),
    Long(w8),
    Float(f32),
    Double(f64),
    String(String)
}
