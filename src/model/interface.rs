use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Interface(pub String);

impl Interface {
    pub fn new(str: String) -> Interface {
        Interface(str)
    }
}
