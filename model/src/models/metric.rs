use serde::{Deserialize, Serialize};
use tiberius_mappers::TryFromRow;

#[derive(Debug, Serialize, Deserialize, Clone, TryFromRow)]
pub struct Metric {
    pub id: i32,
    pub dataset_id: i32,
    pub name: String,
}
