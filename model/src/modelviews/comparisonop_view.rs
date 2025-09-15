use serde::{Deserialize, Serialize};

use crate::shared::enum_share::EnumOperator;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComparisonOpView {
    pub id: i32,
    pub inc_excl: String,
    pub operator: EnumOperator,
    pub setting_version_id: i32,
}
