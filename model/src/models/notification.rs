use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    pub id: i32,
    pub to: String,
    pub cc: String,
    pub bcc: String,
}
