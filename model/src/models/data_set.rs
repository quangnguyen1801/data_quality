use serde::{Deserialize, Serialize};
use tiberius_mappers::TryFromRow;

#[derive(Debug, Serialize, Deserialize, Clone, TryFromRow)]
pub struct Dataset {
    pub id: i32,
    pub data_type: String,
    pub path: String,
    pub name: String,
}
