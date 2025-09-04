use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectorView {
    pub id: i32,
    pub field_name: String,
    pub inc_excl: String,
    pub operator: String,
    pub vlow: String,
    pub vhigh: String,
}
