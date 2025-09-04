use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetricView {
    pub id: i32,
    pub dataset_id: i32,
    pub name: String,
}
