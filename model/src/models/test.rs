use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tiberius_mappers::TryFromRow;

#[derive(Debug, Serialize, Deserialize, Clone, TryFromRow)]
pub struct Test {
    pub id: i32,
    pub group: String,
    pub description: String,
    pub expired: Option<DateTime<Utc>>,
}
