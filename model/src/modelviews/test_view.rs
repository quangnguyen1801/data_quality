use chrono::{DateTime, Utc};

use crate::shared::enum_share::EnumGroup;

pub struct TestView {
    pub id: i32,
    pub group: EnumGroup,
    pub description: String,
    pub expired: Option<DateTime<Utc>>,
}
