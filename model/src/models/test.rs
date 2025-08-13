use chrono::{DateTime, Utc};

use crate::shared::enum_share::EnumGroup;

pub struct Test {
    pub id: i32,
    pub group: EnumGroup,
    pub description: String,
    pub expired: Option<DateTime<Utc>>,
}
