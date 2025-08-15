use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationView {
    pub id: i32,
    pub to: String,
    pub cc: String,
    pub bcc: String,
}
