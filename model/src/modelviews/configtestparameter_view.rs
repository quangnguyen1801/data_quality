use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigTestParameterView {
    pub id: i32,
    pub test_id: i32,
    pub test_parameter_id: i32,
    pub inc_excl: String,
    pub operator: String,
    pub vlow: String,
    pub vhigh: String,
    pub setting_version_id: i32,
    pub ref_columns: i32,
}
