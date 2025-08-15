use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scope {
    pub id: i32,
    pub dataset_id: i32,
    pub selector_id: i32,
    pub metric_id: Vec<i32>,
}
