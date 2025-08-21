use serde::{Deserialize, Serialize};
use tiberius_mappers::TryFromRow;

#[derive(Debug, Serialize, Deserialize, Clone, TryFromRow)]
pub struct Scope {
    pub id: i32,
    pub dataset_id: i32,
    pub selector_id: i32,
    pub metric_id: String,
}
