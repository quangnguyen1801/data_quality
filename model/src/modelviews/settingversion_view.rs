use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SettingVersionView {
    pub id: i32,
    pub file_name: String,
    pub upddate: DateTime<Utc>,
}
