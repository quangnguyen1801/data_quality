use serde::{Deserialize, Serialize};
use tiberius_mappers::TryFromRow;

#[derive(Debug, Serialize, Deserialize, Clone, TryFromRow)]
pub struct Notification {
    pub id: i32,
    pub to: String,
    pub cc: String,
    pub bcc: String,
    pub setting_version_id: i32,
}
