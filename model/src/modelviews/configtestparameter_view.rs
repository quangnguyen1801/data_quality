use serde::{Deserialize, Serialize};

use crate::shared::enum_share::EnumOperator;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigTestParameterView {
    pub id: i32,
    pub test_id: i32,
    pub test_parameter_id: i32,
    pub inc_excl: String,
    pub operator: EnumOperator,
    pub vlow: String,
    pub vhigh: String,
}
