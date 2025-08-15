use serde::{Deserialize, Serialize};

use crate::shared::enum_share::EnumOperator;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComparisonOpView {
    pub inc_excl: String,
    pub operator: EnumOperator,
}
