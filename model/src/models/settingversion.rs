use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tiberius_mappers::TryFromRow;
#[derive(Debug, Clone, Deserialize, Serialize, TryFromRow)]
pub struct SettingVersion {
    pub id: i32,
    pub file_name: String,
    pub upddate: DateTime<Utc>,
}
