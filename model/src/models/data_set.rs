use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dataset {
    pub id: i32,
    pub data_type: String,
    pub path: String,
    pub name: String,
}
