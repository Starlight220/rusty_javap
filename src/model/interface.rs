use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Interface(String);

impl Interface {
    pub fn new(str: String) -> Interface {
        Interface(str)
    }
}
