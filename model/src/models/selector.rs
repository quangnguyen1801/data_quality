use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Selector {
    pub id: i32,
    pub field_name: String,
    pub inc_excl: String,
    pub operator: i32,
    pub vlow: String,
    pub vhigh: String,
}
