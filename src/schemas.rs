use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LacOut {
    pub word: String,
    pub tag: String,
}
